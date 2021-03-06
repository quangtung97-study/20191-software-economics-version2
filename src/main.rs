///////////////////////
// Ta Quang Tung     //
// KSTN - CNTT - K60 //
///////////////////////

mod computation;
mod input;
mod mrgame;
mod newton;
mod relation;
mod rrgame;
mod solver;

use mrgame::MRGame;
use relation::Material;
use relation::Retailer;
use rrgame::RRGame;

fn main() {
    let (relation, constant) = input::input();
    let mut mrgame = MRGame::new(&relation);

    mrgame.decision.input(&relation, &[true, true, true, true]);
    // mrgame.decision.show(&relation);

    mrgame.parameter.input_A_g_c_m(
        &relation,
        &[3787.0, 3562.0, 1000.0, 6200.0],
        &[0.1721, 0.1403],
    );
    mrgame
        .parameter
        .input_crm_s(&relation, &[0.1673, 0.4874, 0.3463]);
    mrgame.parameter.input_drm_sl(
        &relation,
        &[
            &[2000.0, 1000.0, 3000.0],
            &[2000.0, 0.0, 4000.0],
            &[5000.0, 1500.0, 0.0],
        ],
    );
    mrgame.parameter.show(&relation);

    let mut rrgame = RRGame::new(&relation);
    rrgame.parameter.input_p_mg(
        &relation,
        &[&[67.88, 61.02, 0.0, 55.72], &[70.69, 0.0, 0.0, 63.57]],
    );
    rrgame.parameter.show_p_mg(&relation);

    rrgame.parameter.input_a_mg(
        &relation,
        &[
            &[1049.85, 2850.10, 0.0, 631.43],
            &[2509.19, 0.0, 0.0, 1501.64],
        ],
    );
    rrgame.parameter.show_a_mg(&relation);

    {
        let input = computation::Input {
            relation: &relation,
            constant: &constant,
            mrgame: &mrgame,
            rrgame: &rrgame,
        };

        let retailer1 = Retailer { id: 0 };
        for g in relation.products(retailer1, &mrgame.decision) {
            let demand = computation::DP(&input, retailer1, g);
            println!("DP_{}{}: {}", retailer1.id, g.id, demand);
        }

        for m in relation.initial_retailers() {
            println!("TVR_constraint: {}", computation::TVR_constraint(&input, m));
            println!("Ta_constraint: {}", computation::Ta_constraint(&input, m));
        }

        for m in relation.initial_retailers() {
            let profit = computation::NP(&input, m);
            println!("Profit: {}", profit);
        }

        println!("dc_NP0");
        for m in relation.initial_retailers() {
            print!("{}\t", computation::dc_NP0(&input, m));
        }
        println!("");

        println!("dc_NP0_approx");
        for m in relation.initial_retailers() {
            print!("{}\t", computation::dc_NP0_approx(&input, m));
        }
        println!("");

        println!("NP0 = {}", computation::NP0(&input));
        println!(
            "NP0 TVP constraint: {}",
            computation::NP0_TVP_constraint(&input)
        );
        println!("NP0 BOM constraint: ");
        for l in relation.all_materials() {
            print!("{}\t", computation::NP0_bom_constraint(&input, l));
        }
        println!("");
    }

    for _step in 0..2 {
        for m in relation.initial_retailers() {
            println!("----------------------------------");
            let new_parameter = {
                let input = computation::Input {
                    relation: &relation,
                    constant: &constant,
                    rrgame: &rrgame,
                    mrgame: &mrgame,
                };
                solver::rrgame_solve(&input, m)
            };
            match new_parameter {
                Some(new_parameter) => {
                    println!("m: {}", m.id);
                    new_parameter.show_p_mg(&relation);
                    new_parameter.show_a_mg(&relation);

                    for g in relation.products(m, &mrgame.decision) {
                        rrgame.parameter.p_mg[m][g] = new_parameter.p_mg[m][g];
                        rrgame.parameter.a_mg[m][g] = new_parameter.a_mg[m][g];
                    }
                }
                None => {}
            }
        }
    }

    {
        println!("======================================");
        let input = computation::Input {
            relation: &relation,
            constant: &constant,
            rrgame: &rrgame,
            mrgame: &mrgame,
        };

        let constraints = solver::MRGameConstraints {};
        let lambdas = solver::MRGameLambdas::new(&relation);
        let parameter = solver::mrgame_solve_constraints(&input, constraints);

        println!("Old A_g");
        for g in relation.all_products() {
            print!("{}\t", input.mrgame.parameter.A_g[g]);
        }
        println!("");
        if let Some(parameter) = parameter {
            println!("New A_g");
            for g in relation.all_products() {
                print!("{}\t", parameter.A_g[g]);
            }
            println!("");

            println!("New c_m");
            for m in relation.initial_retailers() {
                print!("{}\t", parameter.c_m[m]);
            }
            println!("");

            println!("New crm_s");
            for s in relation.all_suppliers() {
                print!("{}\t", parameter.crm_s[s]);
            }
            println!("");

            println!("Old NP0 = {}", computation::NP0(&input));
            let decision = input.mrgame.decision.clone();
            let mrgame = mrgame::MRGame {
                parameter: parameter,
                decision: decision,
            };
            let new_input = computation::Input {
                mrgame: &mrgame,
                ..input
            };

            println!("New NP0 = {}", computation::NP0(&new_input));

            println!("dA_NP0");
            for j in relation.all_products() {
                print!("{}\t", computation::dA_NP0(&new_input, j));
            }
            println!("");
        }
    }
}
