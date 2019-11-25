use crate::relation::Constant;
use crate::relation::Relation;

fn input_v_mgxy(relation: &Relation, constant: &mut Constant) {
    constant.input_v_mgxy(
        relation,
        0,
        0,
        &[
            &[6.3, -2.1, -1.5, -0.6],
            &[-1.9, -1.6, -1.0, 0.0],
            &[-1.2, 0.0, 0.0, -0.5],
        ],
    );

    constant.input_v_mgxy(
        relation,
        0,
        1,
        &[
            &[-1.6, 6.6, -1.6, -1.0],
            &[-1.0, -1.6, -1.0, 0.0],
            &[-1.0, 0.0, 0.0, -1.0],
        ],
    );

    constant.input_v_mgxy(
        relation,
        0,
        2,
        &[
            &[-1.6, -1.6, 6.3, -1.6],
            &[-1.0, -1.0, -1.6, 0.0],
            &[-0.6, 0.0, 0.0, -1.0],
        ],
    );

    constant.input_v_mgxy(
        relation,
        0,
        3,
        &[
            &[-1.2, -1.5, -1.9, 7.2],
            &[-0.6, -1.0, -1.1, 0.0],
            &[-0.6, 0.0, 0.0, -1.5],
        ],
    );

    constant.input_v_mgxy(
        relation,
        1,
        0,
        &[
            &[-2.6, -1.6, -1.0, -0.6],
            &[6.1, -1.9, -1.3, 0.0],
            &[-1.0, 0.0, 0.0, -0.7],
        ],
    );

    constant.input_v_mgxy(
        relation,
        1,
        1,
        &[
            &[-1.0, -1.6, -1.0, -0.6],
            &[-1.6, -6.3, -1.6, 0.0],
            &[-0.6, 0.0, 0.0, -0.6],
        ],
    );

    constant.input_v_mgxy(
        relation,
        1,
        2,
        &[
            &[-1.0, -1.6, -1.0, -0.6],
            &[-1.6, -6.3, -1.6, 0.0],
            &[-0.6, 0.0, 0.0, -0.6],
        ],
    );

    constant.input_v_mgxy(
        relation,
        2,
        0,
        &[
            &[-1.6, -1.2, -1.0, -0.9],
            &[-1.2, -1.0, -0.7, 0.0],
            &[8.3, 0.0, 0.0, -1.5],
        ],
    );

    constant.input_v_mgxy(
        relation,
        2,
        3,
        &[
            &[-0.7, -0.8, -1.0, -1.5],
            &[-0.5, -0.6, -1.0, 0.0],
            &[-1.0, 0.0, 0.0, 7.2],
        ],
    );

    // constant.show_v_mgxy(relation);
}

fn input_ea_mgxy(relation: &Relation, constant: &mut Constant) {
    constant.input_ea_mgxy(
        relation,
        0,
        0,
        &[
            &[0.39, 0.24, 0.22, 0.2],
            &[0.2, 0.16, 0.17, 0.0],
            &[0.2, 0.0, 0.0, 0.16],
        ],
    );

    constant.input_ea_mgxy(
        relation,
        0,
        1,
        &[
            &[0.23, 0.46, 0.22, 0.18],
            &[0.16, 0.2, 0.16, 0.0],
            &[0.16, 0.0, 0.0, 0.16],
        ],
    );

    constant.input_ea_mgxy(
        relation,
        0,
        2,
        &[
            &[0.2, 0.2, 0.37, 0.2],
            &[0.18, 0.18, 0.2, 0.0],
            &[0.18, 0.0, 0.0, 0.18],
        ],
    );

    constant.input_ea_mgxy(
        relation,
        0,
        3,
        &[
            &[0.19, 0.2, 0.23, 0.42],
            &[0.2, 0.16, 0.17, 0.0],
            &[0.6, 0.0, 0.0, 0.22],
        ],
    );

    constant.input_ea_mgxy(
        relation,
        1,
        0,
        &[
            &[0.36, 0.3, 0.29, 0.18],
            &[0.4, 0.33, 0.3, 0.0],
            &[0.2, 0.0, 0.0, 0.16],
        ],
    );

    constant.input_ea_mgxy(
        relation,
        1,
        1,
        &[
            &[0.34, 0.36, 0.3, 0.24],
            &[0.4, 0.41, 0.32, 0.0],
            &[0.16, 0.0, 0.0, 0.16],
        ],
    );

    constant.input_ea_mgxy(
        relation,
        1,
        2,
        &[
            &[0.24, 0.32, 0.29, 0.28],
            &[0.32, 0.34, 0.37, 0.0],
            &[0.16, 0.0, 0.0, 0.2],
        ],
    );

    constant.input_ea_mgxy(
        relation,
        2,
        0,
        &[
            &[0.36, 0.31, 0.3, 0.26],
            &[0.2, 0.16, 0.14, 0.0],
            &[0.4, 0.0, 0.0, 0.29],
        ],
    );

    constant.input_ea_mgxy(
        relation,
        2,
        3,
        &[
            &[0.2, 0.32, 0.38, 0.42],
            &[0.13, 0.18, 0.24, 0.0],
            &[0.2, 0.0, 0.0, 0.43],
        ],
    );

    // constant.show_ea_mgxy(relation);
}

fn input_beta_mgxy(relation: &Relation, constant: &mut Constant) {
    constant.input_beta_mgxy(
        relation,
        0,
        0,
        &[
            &[-19.0, 2.2, 2.1, 1.2],
            &[2.5, 1.6, 1.2, 0.0],
            &[2.1, 0.0, 0.0, 0.8],
        ],
    );

    constant.input_beta_mgxy(
        relation,
        0,
        1,
        &[
            &[1.8, -25.0, 1.9, 1.5],
            &[0.9, 2.2, 1.1, 0.0],
            &[1.2, 0.0, 0.0, 0.7],
        ],
    );

    constant.input_beta_mgxy(
        relation,
        0,
        2,
        &[
            &[1.5, 1.9, -22.0, 2.0],
            &[1.0, 1.0, 21.0, 0.0],
            &[1.1, 0.0, 0.0, 1.6],
        ],
    );

    constant.input_beta_mgxy(
        relation,
        0,
        3,
        &[
            &[1.3, 18.0, 2.5, -26.0],
            &[0.7, 1.0, 18.0, 0.0],
            &[0.6, 0.0, 0.0, 2.1],
        ],
    );

    constant.input_beta_mgxy(
        relation,
        1,
        0,
        &[
            &[2.4, 1.6, 1.0, 0.7],
            &[-19.0, 1.9, 1.3, 0.0],
            &[1.0, 0.0, 0.0, 0.6],
        ],
    );

    constant.input_beta_mgxy(
        relation,
        1,
        1,
        &[
            &[1.0, 2.1, 1.2, 0.8],
            &[1.6, 1.8, 1.6, 0.0],
            &[1.1, 0.0, 0.0, 0.5],
        ],
    );

    constant.input_beta_mgxy(
        relation,
        1,
        2,
        &[
            &[0.8, 1.1, 1.8, 1.0],
            &[15.0, 25.0, -25.0, 0.0],
            &[0.6, 0.0, 0.0, 0.7],
        ],
    );

    constant.input_beta_mgxy(
        relation,
        2,
        0,
        &[
            &[2.3, 1.8, 1.1, 0.8],
            &[1.1, 1.0, 0.7, 0.0],
            &[-18.0, 0.0, 0.0, 1.9],
        ],
    );

    constant.input_beta_mgxy(
        relation,
        2,
        3,
        &[
            &[1.0, 1.5, 2.1, 2.2],
            &[0.5, 0.6, 0.7, 0.0],
            &[1.1, 0.0, 0.0, -23.0],
        ],
    );

    // constant.show_beta_mgxy(relation);
}

fn input_K_mg(relation: &Relation, constant: &mut Constant) {
    constant.input_K_mg(
        &relation,
        &[
            &[700.0, 800.0, 800.0, 900.0],
            &[700.0, 900.0, 700.0, 0.0],
            &[900.0, 0.0, 0.0, 800.0],
        ],
    );
    // constant.show_K_mg(&relation);
}

fn input_eA_mgy(relation: &Relation, constant: &mut Constant) {
    constant.input_aA_mgy(
        relation,
        &[
            &[0.35, 0.26, 0.2, 0.18],
            &[0.21, 0.36, 0.21, 0.15],
            &[0.17, 0.22, 0.34, 0.21],
            &[0.1, 0.19, 0.21, 0.36],
            &[0.32, 0.25, 0.12, 0.0],
            &[0.2, 0.33, 0.2, 0.0],
            &[0.17, 0.23, 0.35, 0.0],
            &[0.32, 0.0, 0.0, 0.2],
            &[0.23, 0.0, 0.0, 0.36],
        ],
    );
    // constant.show_eA_mgy(relation);
}

fn input_u_mgy(relation: &Relation, constant: &mut Constant) {
    constant.input_u_mgy(
        relation,
        &[
            &[12.0, -1.8, -1.5, -1.1],
            &[-1.7, 18.0, -1.2, -0.9],
            &[-1.0, -1.9, 11.0, -1.9],
            &[-0.9, -1.1, -2.1, 16.0],
            &[12.0, -2.2, -1.0, 0.0],
            &[-1.7, 18.0, -1.8, 0.0],
            &[-1.7, -1.9, 11.0, 0.0],
            &[11.0, 0.0, 0.0, -1.0],
            &[-1.3, 0.0, 0.0, 18.0],
        ],
    );
    // constant.show_u_mgy(relation);
}

pub fn input() -> (Relation, Constant) {
    let mut relation = Relation::new();

    relation.supplier_material_pairs(&[(0, 0), (0, 1), (0, 2), (1, 0), (1, 2), (2, 0), (2, 1)]);

    relation.retailer_product_pairs(&[
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 0),
        (1, 1),
        (1, 2),
        (2, 0),
        (2, 3),
    ]);

    relation.alternative_module_pairs(&[(0, 0), (1, 0), (2, 0), (3, 1), (4, 1), (5, 1)]);

    relation.material_alternative_pairs(&[
        (0, 0),
        (0, 1),
        (0, 2),
        (0, 3),
        (1, 0),
        (1, 1),
        (1, 4),
        (2, 2),
        (2, 5),
    ]);

    relation.alternative_product_pairs(&[
        (0, 0),
        (1, 1),
        (1, 2),
        (2, 3),
        (3, 0),
        (3, 2),
        (4, 1),
        (5, 3),
    ]);

    let mut constant = Constant::new(&relation);

    input_v_mgxy(&relation, &mut constant);
    input_ea_mgxy(&relation, &mut constant);
    input_beta_mgxy(&relation, &mut constant);
    input_K_mg(&relation, &mut constant);
    input_eA_mgy(&relation, &mut constant);
    input_u_mgy(&relation, &mut constant);

    // constant.show_ep_mgxy(&relation);

    (relation, constant)
}
