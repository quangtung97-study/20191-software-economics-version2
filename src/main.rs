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

use mrgame::MRGame;
use relation::{Product, Retailer};
use rrgame::RRGame;

fn main() {
    let (relation, constant) = input::input();
    let mut mrgame = MRGame::new(&relation);

    mrgame.decision.input(&relation, &[true, true, false, true]);
    // mrgame.decision.show(&relation);

    mrgame.parameter.input_A_g_c_m(
        &relation,
        &[3787.0, 3562.0, 0.0, 6200.0],
        &[0.1721, 0.1830, 0.1403],
    );
    mrgame
        .parameter
        .input_crm_s(&relation, &[0.1673, 0.4874, 0.3463]);
    mrgame.parameter.show(&relation);

    let mut rrgame = RRGame::new(&relation);
    rrgame.parameter.input_p_mg(
        &relation,
        &[
            &[67.88, 61.02, 0.0, 55.72],
            &[59.70, 69.76, 0.0, 0.0],
            &[70.69, 0.0, 0.0, 63.57],
        ],
    );
    rrgame.parameter.show_p_mg(&relation);

    rrgame.parameter.input_a_mg(
        &relation,
        &[
            &[1049.85, 2850.10, 0.0, 631.43],
            &[357.29, 1681.27, 0.0, 0.0],
            &[2509.19, 0.0, 0.0, 1501.64],
        ],
    );
    rrgame.parameter.show_a_mg(&relation);

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
        for j in relation.products(m, &mrgame.decision) {
            print!("{}\t", computation::da_NP(&input, m, j));
        }
    }
    println!("");

    for m in relation.initial_retailers() {
        for j in relation.products(m, &mrgame.decision) {
            print!("{}\t", computation::da_NP_approx(&input, m, j));
        }
    }
    println!("");

    for m in relation.initial_retailers() {
        for j in relation.products(m, &mrgame.decision) {
            print!("{}\t", computation::dpdp_NP(&input, m, j, j));
        }
    }
    println!("");

    for m in relation.initial_retailers() {
        let profit = computation::NP(&input, m);
        println!("Profit: {}", profit);
    }

    println!("NP0 = {}", computation::NP0(&input));
}
