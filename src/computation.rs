use crate::mrgame::MRGame;
use crate::relation::Alternative;
use crate::relation::Constant;
use crate::relation::Material;
use crate::relation::Product;
use crate::relation::Relation;
use crate::relation::Retailer;
use crate::rrgame::RRGame;

pub struct Input<'a, 'b, 'c, 'd> {
    pub relation: &'a Relation,
    pub constant: &'b Constant,
    pub mrgame: &'c MRGame,
    pub rrgame: &'d RRGame,
}

fn safe_pow(a: f64, n: f64) -> f64 {
    if n == 0.0 {
        1.0
    } else if a < 0.0 {
        0.0
    } else {
        f64::powf(a, n)
    }
}

pub fn DP(input: &Input, m: Retailer, g: Product) -> f64 {
    let K_mg = &input.constant.K_mg;
    let u_mgy = &input.constant.u_mgy;
    let eA_mgy = &input.constant.eA_mgy;
    let A_g = &input.mrgame.parameter.A_g;
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let beta_mgxy = &input.constant.beta_mgxy;
    let ep_mgxy = &input.constant.ep_mgxy;

    let v_mgxy = &input.constant.v_mgxy;
    let ea_mgxy = &input.constant.ea_mgxy;

    let p_mg = &input.rrgame.parameter.p_mg;
    let a_mg = &input.rrgame.parameter.a_mg;

    let mut sum = K_mg[m][g];
    for y in relation.products(m, decision) {
        sum += u_mgy[m][g][y] * safe_pow(A_g[y], eA_mgy[m][g][y]);
    }

    for x in relation.initial_retailers() {
        for y in relation.products(x, decision) {
            sum += beta_mgxy[m][g][x][y] * safe_pow(p_mg[x][y], ep_mgxy[m][g][x][y]);
            sum += v_mgxy[m][g][x][y] * safe_pow(a_mg[x][y], ea_mgxy[m][g][x][y]);
        }
    }

    sum
}

pub fn pw(input: &Input, m: Retailer, g: Product) -> f64 {
    let pw_g0 = &input.constant.pw_g0;
    let rho_g = &input.mrgame.parameter.rho_g;

    pw_g0[g] - rho_g[g] * DP(input, m, g)
}

pub fn NP(input: &Input, m: Retailer) -> f64 {
    let p_mg = &input.rrgame.parameter.p_mg;
    let a_mg = &input.rrgame.parameter.a_mg;
    let zeta_mg = &input.constant.zeta_mg;
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let mut sum = 0.0;

    for g in relation.products(m, decision) {
        sum += DP(input, m, g) * p_mg[m][g];
        sum -= DP(input, m, g) * pw(input, m, g);
        sum -= zeta_mg[m][g] * DP(input, m, g);
        sum -= a_mg[m][g];
    }

    sum
}

fn DA(input: &Input, k: Alternative) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let delta_gk = &input.constant.delta_gk;

    let mut sum = 0.0;
    for g in relation.products_for_alternative(k, &decision) {
        for m in relation.retailers(g) {
            sum += (delta_gk[g][k] as f64) * DP(input, m, g);
        }
    }

    sum
}

pub fn NP0(input: &Input) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let c_m = &input.mrgame.parameter.c_m;
    let HR_mg = &input.constant.HR_mg;
    let zeta_mg = &input.constant.zeta_mg;
    let TP_mg = &input.constant.TP_mg;
    let PCP_g = &input.constant.PCP_g;
    let ORM_s = &input.constant.ORM_s;
    let HRM_l = &input.constant.HRM_l;
    let PCA_k = &input.constant.PCA_k;
    let PCR_sl = &input.constant.PCR_sl;
    let FCM_j = &input.constant.FCM_j;
    let FCA_k = &input.constant.FCA_k;

    let crm_s = &input.mrgame.parameter.crm_s;
    let drm_sl = &input.mrgame.parameter.drm_sl;
    let A_g = &input.mrgame.parameter.A_g;

    let mut sum = 0.0;

    for m in relation.initial_retailers() {
        for g in relation.products(m, decision) {
            sum += DP(input, m, g) * pw(input, m, g);
        }
    }

    sum -= {
        let mut inner_sum = 0.0;

        for m in relation.initial_retailers() {
            for g in relation.products(m, decision) {
                inner_sum += c_m[m] * DP(input, m, g) * HR_mg[m][g];
                inner_sum -= zeta_mg[m][g] * DP(input, m, g);
            }
        }

        inner_sum
    };

    sum -= {
        let mut inner_sum = 0.0;
        for s in relation.all_suppliers() {
            inner_sum += ORM_s[s] / crm_s[s];

            for l in relation.materials(s) {
                inner_sum += crm_s[s] * drm_sl[s][l] * HRM_l[l] / 2.0;
            }
        }
        inner_sum
    };

    for m in relation.initial_retailers() {
        for g in relation.products(m, decision) {
            sum -= DP(input, m, g) * TP_mg[m][g];
            sum -= DP(input, m, g) * PCP_g[g];
        }
    }

    for k in relation.all_alternatives() {
        sum -= DA(input, k) * PCA_k[k];
    }

    for s in relation.all_suppliers() {
        for l in relation.all_materials() {
            sum -= drm_sl[s][l] * PCR_sl[s][l];
        }
    }
    // println!("NP0 sum = {}", sum);

    for g in relation.all_products() {
        sum -= decision.fpp(g) * PCP_g[g];
    }

    for j in relation.all_modules() {
        sum -= decision.fpm(relation, j) * FCM_j[j];
    }

    for k in relation.all_alternatives() {
        sum -= decision.fpa(k) * FCA_k[k];
    }

    for g in relation.all_products() {
        sum -= A_g[g];
    }

    sum
}

pub fn NP0_constraint1(input: &Input, l: Material) -> f64 {
    let relation = input.relation;
    let drm_sl = &input.mrgame.parameter.drm_sl;
    let sigma_kl = &input.constant.sigma_kl;

    let mut sum = 0.0;
    for s in relation.all_suppliers() {
        sum += drm_sl[s][l];
    }

    for k in relation.all_alternatives() {
        sum -= sigma_kl[k][l] as f64 * DA(input, k);
    }

    sum
}

pub fn dp_DP(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let p_mg = &input.rrgame.parameter.p_mg;
    let beta_mgxy = &input.constant.beta_mgxy;
    let ep_mgxy = &input.constant.ep_mgxy;

    beta_mgxy[m][g][m][j] * ep_mgxy[m][g][m][j] * safe_pow(p_mg[m][j], ep_mgxy[m][g][m][j] - 1.0)
}

#[allow(dead_code)]
pub fn dp_DP_approx(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let mut rrgame = input.rrgame.clone();
    rrgame.parameter.p_mg[m][j] += 0.01;

    let new_input = Input {
        rrgame: &rrgame,
        ..(*input)
    };

    (DP(&new_input, m, g) - DP(input, m, g)) / 0.01
}

pub fn da_DP(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let a_mg = &input.rrgame.parameter.a_mg;
    let v_mgxy = &input.constant.v_mgxy;
    let ea_mgxy = &input.constant.ea_mgxy;

    v_mgxy[m][g][m][j] * ea_mgxy[m][g][m][j] * safe_pow(a_mg[m][j], ea_mgxy[m][g][m][j] - 1.0)
}

#[allow(dead_code)]
pub fn da_DP_approx(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let mut rrgame = input.rrgame.clone();
    rrgame.parameter.a_mg[m][j] += 0.01;

    let new_input = Input {
        rrgame: &rrgame,
        ..(*input)
    };

    (DP(&new_input, m, g) - DP(input, m, g)) / 0.01
}

pub fn dp_pw(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let rho_g = &input.mrgame.parameter.rho_g;
    -rho_g[g] * dp_DP(input, m, g, j)
}

pub fn dp_NP(input: &Input, m: Retailer, j: Product) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;
    let p_mg = &input.rrgame.parameter.p_mg;
    let zeta_mg = &input.constant.zeta_mg;

    let mut sum = DP(input, m, j);
    for g in relation.products(m, decision) {
        sum += dp_DP(input, m, g, j) * p_mg[m][g];

        sum -= dp_DP(input, m, g, j) * pw(input, m, g);
        sum -= DP(input, m, g) * dp_pw(input, m, g, j);

        sum -= zeta_mg[m][g] * dp_DP(input, m, g, j);
    }

    sum
}

#[allow(dead_code)]
pub fn dp_NP_approx(input: &Input, m: Retailer, j: Product) -> f64 {
    let mut rrgame = input.rrgame.clone();
    rrgame.parameter.p_mg[m][j] += 0.01;

    let new_input = Input {
        rrgame: &rrgame,
        ..(*input)
    };

    (NP(&new_input, m) - NP(input, m)) / 0.01
}

pub fn da_pw(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let rho_g = &input.mrgame.parameter.rho_g;
    -rho_g[g] * da_DP(input, m, g, j)
}

#[allow(dead_code)]
pub fn da_pw_approx(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let mut rrgame = input.rrgame.clone();
    rrgame.parameter.a_mg[m][j] += 0.01;

    let new_input = Input {
        rrgame: &rrgame,
        ..(*input)
    };

    (pw(&new_input, m, g) - pw(input, m, g)) / 0.01
}

pub fn da_NP(input: &Input, m: Retailer, j: Product) -> f64 {
    let p_mg = &input.rrgame.parameter.p_mg;
    let relation = input.relation;
    let decision = &input.mrgame.decision;
    let zeta_mg = &input.constant.zeta_mg;

    let mut sum = 0.0;

    for g in relation.products(m, decision) {
        sum += da_DP(input, m, g, j) * p_mg[m][g];
        sum -= da_DP(input, m, g, j) * pw(input, m, g);
        sum -= DP(input, m, g) * da_pw(input, m, g, j);
        sum -= zeta_mg[m][g] * da_DP(input, m, g, j);
    }

    sum -= 1.0;

    sum
}

#[allow(dead_code)]
pub fn da_NP_approx(input: &Input, m: Retailer, j: Product) -> f64 {
    let mut rrgame = input.rrgame.clone();
    let delta = 0.001;
    rrgame.parameter.a_mg[m][j] += delta;

    let new_input = Input {
        rrgame: &rrgame,
        ..(*input)
    };

    (NP(&new_input, m) - NP(input, m)) / delta
}

pub fn dpdp_NP(input: &Input, m: Retailer, j: Product, k: Product) -> f64 {
    let mut rrgame = input.rrgame.clone();
    let delta = 0.0001;
    rrgame.parameter.p_mg[m][k] += delta;

    let new_input = Input {
        rrgame: &rrgame,
        ..(*input)
    };

    (dp_NP(&new_input, m, j) - dp_NP(input, m, j)) / delta
}

#[allow(dead_code)]
pub fn compute_TVR_m(input: &Input, m: Retailer) -> f64 {
    let w_m = &input.constant.w_m;
    let c_m = &input.mrgame.parameter.c_m;
    let V_g = &input.constant.V_g;
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let mut sum = 0.0;
    for g in relation.products(m, decision) {
        sum += c_m[m] * DP(input, m, g) * V_g[g] / 2.0;
    }

    w_m[m] * sum
}

pub fn TVR_constraint(input: &Input, m: Retailer) -> f64 {
    let w_m = &input.constant.w_m;
    let c_m = &input.mrgame.parameter.c_m;
    let V_g = &input.constant.V_g;
    let relation = input.relation;
    let decision = &input.mrgame.decision;
    let TVR_m = &input.constant.TVR_m;

    let mut sum = 0.0;
    for g in relation.products(m, decision) {
        sum += c_m[m] * DP(input, m, g) * V_g[g] / 2.0;
    }

    w_m[m] * sum - TVR_m[m]
}

pub fn dp_TVR_constraint(input: &Input, m: Retailer, j: Product) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;
    let w_m = &input.constant.w_m;
    let c_m = &input.mrgame.parameter.c_m;
    let V_g = &input.constant.V_g;

    let mut sum = 0.0;

    for g in relation.products(m, decision) {
        sum += c_m[m] * dp_DP(input, m, g, j) * V_g[g] / 2.0;
    }

    w_m[m] * sum
}

#[allow(dead_code)]
pub fn dp_TVR_constraint_approx(input: &Input, m: Retailer, j: Product) -> f64 {
    let mut rrgame = input.rrgame.clone();
    let delta = 0.001;
    rrgame.parameter.p_mg[m][j] += delta;

    let new_input = Input {
        rrgame: &rrgame,
        ..(*input)
    };

    (TVR_constraint(&new_input, m) - TVR_constraint(input, m)) / delta
}

pub fn da_TVR_constraint(input: &Input, m: Retailer, j: Product) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;
    let w_m = &input.constant.w_m;
    let c_m = &input.mrgame.parameter.c_m;
    let V_g = &input.constant.V_g;

    let mut sum = 0.0;

    for g in relation.products(m, decision) {
        sum += c_m[m] * da_DP(input, m, g, j) * V_g[g] / 2.0;
    }

    w_m[m] * sum
}

#[allow(dead_code)]
pub fn da_TVR_constraint_approx(input: &Input, m: Retailer, j: Product) -> f64 {
    let mut rrgame = input.rrgame.clone();
    let delta = 0.001;
    rrgame.parameter.a_mg[m][j] += delta;

    let new_input = Input {
        rrgame: &rrgame,
        ..(*input)
    };

    (TVR_constraint(&new_input, m) - TVR_constraint(input, m)) / delta
}

pub fn Ta_constraint(input: &Input, m: Retailer) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;
    let a_mg = &input.rrgame.parameter.a_mg;
    let Ta_m = &input.constant.Ta_m;

    let mut sum = 0.0;
    for g in relation.products(m, decision) {
        sum += a_mg[m][g];
    }

    sum - Ta_m[m]
}

pub fn da_Ta_constraint(input: &Input, _m: Retailer, _j: Product) -> f64 {
    1.0
}
