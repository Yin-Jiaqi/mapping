use egraph_mapping::{SimpleLanguage};
use egg::*;

pub fn external_rules() -> Vec<Rewrite<SimpleLanguage, ()>> {
    let rules=vec![
        rewrite!("external_rule_0";"(not ?A)" => "(INVx11_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_1";"(not ?A)" => "(INVx13_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_2";"(not ?A)" => "(INVx1_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_3";"(not ?A)" => "(INVx2_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_4";"(not ?A)" => "(INVx3_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_5";"(not ?A)" => "(INVx4_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_6";"(not ?A)" => "(INVx5_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_7";"(not ?A)" => "(INVx6_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_8";"(not ?A)" => "(INVx8_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_9";"(not ?A)" => "(INVxp33_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_10";"(not ?A)" => "(INVxp67_ASAP7_75t_L ?A)"),
        rewrite!("external_rule_11";"(and ?A ?B)" => "(AND2x2_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_12";"(and ?A ?B)" => "(AND2x4_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_13";"(and ?A ?B)" => "(AND2x6_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_14";"(and ?A ?B ?C)" => "(AND3x1_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_15";"(and ?A ?B ?C)" => "(AND3x2_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_16";"(and ?A ?B ?C)" => "(AND3x4_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_17";"(and ?A ?B ?C ?D)" => "(AND4x1_ASAP7_75t_L ?A ?B ?C ?D)"),
        rewrite!("external_rule_18";"(and ?A ?B ?C ?D)" => "(AND4x2_ASAP7_75t_L ?A ?B ?C ?D)"),
        rewrite!("external_rule_19";"(and ?A ?B ?C ?D ?E)" => "(AND5x1_ASAP7_75t_L ?A ?B ?C ?D ?E)"),
        rewrite!("external_rule_20";"(and ?A ?B ?C ?D ?E)" => "(AND5x2_ASAP7_75t_L ?A ?B ?C ?D ?E)"),
        rewrite!("external_rule_21";"(or (and (not ?A) (not ?B)) (and (not ?A) (not ?C)) (and (not ?B) (not ?C)))" => "(MAJIxp5_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_22";"(or (and ?A ?B) (and ?A ?C) (and ?B ?C))" => "(MAJx2_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_23";"(or (and ?A ?B) (and ?A ?C) (and ?B ?C))" => "(MAJx3_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_24";"(or (not ?A) (not ?B))" => "(NAND2x1_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_25";"(or (not ?A) (not ?B))" => "(NAND2x1p5_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_26";"(or (not ?A) (not ?B))" => "(NAND2x2_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_27";"(or (not ?A) (not ?B))" => "(NAND2xp33_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_28";"(or (not ?A) (not ?B))" => "(NAND2xp5_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_29";"(or (not ?A) (not ?B))" => "(NAND2xp67_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_30";"(or (not ?A) (not ?B) (not ?C))" => "(NAND3x1_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_31";"(or (not ?A) (not ?B) (not ?C))" => "(NAND3x2_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_32";"(or (not ?A) (not ?B) (not ?C))" => "(NAND3xp33_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_33";"(or (not ?A) (not ?B) (not ?C) (not ?D))" => "(NAND4xp25_ASAP7_75t_L ?A ?B ?C ?D)"),
        rewrite!("external_rule_34";"(or (not ?A) (not ?B) (not ?C) (not ?D))" => "(NAND4xp75_ASAP7_75t_L ?A ?B ?C ?D)"),
        rewrite!("external_rule_35";"(or (not ?A) (not ?B) (not ?C) (not ?D) (not ?E))" => "(NAND5xp2_ASAP7_75t_L ?A ?B ?C ?D ?E)"),
        rewrite!("external_rule_36";"(and (not ?A) (not ?B))" => "(NOR2x1_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_37";"(and (not ?A) (not ?B))" => "(NOR2x1p5_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_38";"(and (not ?A) (not ?B))" => "(NOR2x2_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_39";"(and (not ?A) (not ?B))" => "(NOR2xp33_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_40";"(and (not ?A) (not ?B))" => "(NOR2xp67_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_41";"(and (not ?A) (not ?B) (not ?C))" => "(NOR3x1_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_42";"(and (not ?A) (not ?B) (not ?C))" => "(NOR3x2_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_43";"(and (not ?A) (not ?B) (not ?C))" => "(NOR3xp33_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_44";"(and (not ?A) (not ?B) (not ?C) (not ?D))" => "(NOR4xp25_ASAP7_75t_L ?A ?B ?C ?D)"),
        rewrite!("external_rule_45";"(and (not ?A) (not ?B) (not ?C) (not ?D))" => "(NOR4xp75_ASAP7_75t_L ?A ?B ?C ?D)"),
        rewrite!("external_rule_46";"(and (not ?A) (not ?B) (not ?C) (not ?D) (not ?E))" => "(NOR5xp2_ASAP7_75t_L ?A ?B ?C ?D ?E)"),
        rewrite!("external_rule_47";"(or ?A ?B)" => "(OR2x2_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_48";"(or ?A ?B)" => "(OR2x4_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_49";"(or ?A ?B)" => "(OR2x6_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_50";"(or ?A ?B ?C)" => "(OR3x1_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_51";"(or ?A ?B ?C)" => "(OR3x2_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_52";"(or ?A ?B ?C)" => "(OR3x4_ASAP7_75t_L ?A ?B ?C)"),
        rewrite!("external_rule_53";"(or ?A ?B ?C ?D)" => "(OR4x1_ASAP7_75t_L ?A ?B ?C ?D)"),
        rewrite!("external_rule_54";"(or ?A ?B ?C ?D)" => "(OR4x2_ASAP7_75t_L ?A ?B ?C ?D)"),
        rewrite!("external_rule_55";"(or ?A ?B ?C ?D ?E)" => "(OR5x1_ASAP7_75t_L ?A ?B ?C ?D ?E)"),
        rewrite!("external_rule_56";"(or ?A ?B ?C ?D ?E)" => "(OR5x2_ASAP7_75t_L ?A ?B ?C ?D ?E)"),
        rewrite!("external_rule_57";"(or (and ?A ?B) (and (not ?A) (not ?B)))" => "(XNOR2x1_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_58";"(or (and ?A ?B) (and (not ?A) (not ?B)))" => "(XNOR2x2_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_59";"(or (and ?A ?B) (and (not ?A) (not ?B)))" => "(XNOR2xp5_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_60";"(or (and ?A (not ?B)) (and (not ?A) ?B))" => "(XOR2x1_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_61";"(or (and ?A (not ?B)) (and (not ?A) ?B))" => "(XOR2x2_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_62";"(or (and ?A (not ?B)) (and (not ?A) ?B))" => "(XOR2xp5_ASAP7_75t_L ?A ?B)"),
        rewrite!("external_rule_63";"(or (and ?A1 ?A2) ?B ?C)" => "(AO211x2_ASAP7_75t_L ?A1 ?A2 ?B ?C)"),
        rewrite!("external_rule_64";"(or (and ?A1 ?A2) ?B)" => "(AO21x1_ASAP7_75t_L ?A1 ?A2 ?B)"),
        rewrite!("external_rule_65";"(or (and ?A1 ?A2) ?B)" => "(AO21x2_ASAP7_75t_L ?A1 ?A2 ?B)"),
        rewrite!("external_rule_66";"(or (and ?A1 ?A2) (and ?B1 ?B2) ?C)" => "(AO221x1_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2 ?C)"),
        rewrite!("external_rule_67";"(or (and ?A1 ?A2) (and ?B1 ?B2) ?C)" => "(AO221x2_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2 ?C)"),
        rewrite!("external_rule_68";"(or (and ?A1 ?A2) (and ?B1 ?B2) (and ?C1 ?C2))" => "(AO222x2_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2 ?C1 ?C2)"),
        rewrite!("external_rule_69";"(or (and ?A1 ?A2) (and ?B1 ?B2))" => "(AO22x1_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2)"),
        rewrite!("external_rule_70";"(or (and ?A1 ?A2) (and ?B1 ?B2))" => "(AO22x2_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2)"),
        rewrite!("external_rule_71";"(or (and ?A1 ?A2 ?A3) ?B)" => "(AO31x2_ASAP7_75t_L ?A1 ?A2 ?A3 ?B)"),
        rewrite!("external_rule_72";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2) (and ?C1 ?C2))" => "(AO322x2_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?C1 ?C2)"),
        rewrite!("external_rule_73";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2))" => "(AO32x1_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2)"),
        rewrite!("external_rule_74";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2))" => "(AO32x2_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2)"),
        rewrite!("external_rule_75";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2 ?B3) ?C)" => "(AO331x1_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C)"),
        rewrite!("external_rule_76";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2 ?B3) ?C)" => "(AO331x2_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C)"),
        rewrite!("external_rule_77";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2 ?B3) (and ?C1 ?C2))" => "(AO332x1_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C1 ?C2)"),
        rewrite!("external_rule_78";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2 ?B3) (and ?C1 ?C2))" => "(AO332x2_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C1 ?C2)"),
        rewrite!("external_rule_79";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2 ?B3) (and ?C1 ?C2 ?C3))" => "(AO333x1_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C1 ?C2 ?C3)"),
        rewrite!("external_rule_80";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2 ?B3) (and ?C1 ?C2 ?C3))" => "(AO333x2_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C1 ?C2 ?C3)"),
        rewrite!("external_rule_81";"(or (and ?A1 ?A2 ?A3) (and ?B1 ?B2 ?B3))" => "(AO33x2_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3)"),
        rewrite!("external_rule_82";"(or (and (not ?A1) (not ?B) (not ?C)) (and (not ?A2) (not ?B) (not ?C)))" => "(AOI211x1_ASAP7_75t_L ?A1 ?B ?C ?A2)"),
        rewrite!("external_rule_83";"(or (and (not ?A1) (not ?B) (not ?C)) (and (not ?A2) (not ?B) (not ?C)))" => "(AOI211xp5_ASAP7_75t_L ?A1 ?B ?C ?A2)"),
        rewrite!("external_rule_84";"(or (and (not ?A1) (not ?B)) (and (not ?A2) (not ?B)))" => "(AOI21x1_ASAP7_75t_L ?A1 ?B ?A2)"),
        rewrite!("external_rule_85";"(or (and (not ?A1) (not ?B)) (and (not ?A2) (not ?B)))" => "(AOI21xp33_ASAP7_75t_L ?A1 ?B ?A2)"),
        rewrite!("external_rule_86";"(or (and (not ?A1) (not ?B)) (and (not ?A2) (not ?B)))" => "(AOI21xp5_ASAP7_75t_L ?A1 ?B ?A2)"),
        rewrite!("external_rule_87";"(or (and (not ?A1) (not ?B1) (not ?C)) (and (not ?A1) (not ?B2) (not ?C)) (and (not ?A2) (not ?B1) (not ?C)) (and (not ?A2) (not ?B2) (not ?C)))" => "(AOI221x1_ASAP7_75t_L ?A1 ?B1 ?C ?B2 ?A2)"),
        rewrite!("external_rule_88";"(or (and (not ?A1) (not ?B1) (not ?C)) (and (not ?A1) (not ?B2) (not ?C)) (and (not ?A2) (not ?B1) (not ?C)) (and (not ?A2) (not ?B2) (not ?C)))" => "(AOI221xp5_ASAP7_75t_L ?A1 ?B1 ?C ?B2 ?A2)"),
        rewrite!("external_rule_89";"(or (and (not ?A1) (not ?B1) (not ?C1)) (and (not ?A1) (not ?B1) (not ?C2)) (and (not ?A1) (not ?B2) (not ?C1)) (and (not ?A1) (not ?B2) (not ?C2)) (and (not ?A2) (not ?B1) (not ?C1)) (and (not ?A2) (not ?B1) (not ?C2)) (and (not ?A2) (not ?B2) (not ?C1)) (and (not ?A2) (not ?B2) (not ?C2)))" => "(AOI222xp33_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?B2 ?A2)"),
        rewrite!("external_rule_90";"(or (and (not ?A1) (not ?B1)) (and (not ?A1) (not ?B2)) (and (not ?A2) (not ?B1)) (and (not ?A2) (not ?B2)))" => "(AOI22x1_ASAP7_75t_L ?A1 ?B1 ?B2 ?A2)"),
        rewrite!("external_rule_91";"(or (and (not ?A1) (not ?B1)) (and (not ?A1) (not ?B2)) (and (not ?A2) (not ?B1)) (and (not ?A2) (not ?B2)))" => "(AOI22xp33_ASAP7_75t_L ?A1 ?B1 ?B2 ?A2)"),
        rewrite!("external_rule_92";"(or (and (not ?A1) (not ?B1)) (and (not ?A1) (not ?B2)) (and (not ?A2) (not ?B1)) (and (not ?A2) (not ?B2)))" => "(AOI22xp5_ASAP7_75t_L ?A1 ?B1 ?B2 ?A2)"),
        rewrite!("external_rule_93";"(or (and (not ?A1) (not ?B) (not ?C)) (and (not ?A2) (not ?B) (not ?C)) (and (not ?A3) (not ?B) (not ?C)))" => "(AOI311xp33_ASAP7_75t_L ?A1 ?B ?C ?A2 ?A3)"),
        rewrite!("external_rule_94";"(or (and (not ?A1) (not ?B)) (and (not ?A2) (not ?B)) (and (not ?A3) (not ?B)))" => "(AOI31xp33_ASAP7_75t_L ?A1 ?B ?A2 ?A3)"),
        rewrite!("external_rule_95";"(or (and (not ?A1) (not ?B)) (and (not ?A2) (not ?B)) (and (not ?A3) (not ?B)))" => "(AOI31xp67_ASAP7_75t_L ?A1 ?B ?A2 ?A3)"),
        rewrite!("external_rule_96";"(or (and (not ?A1) (not ?B1) (not ?C)) (and (not ?A1) (not ?B2) (not ?C)) (and (not ?A2) (not ?B1) (not ?C)) (and (not ?A2) (not ?B2) (not ?C)) (and (not ?A3) (not ?B1) (not ?C)) (and (not ?A3) (not ?B2) (not ?C)))" => "(AOI321xp33_ASAP7_75t_L ?A1 ?B1 ?C ?B2 ?A2 ?A3)"),
        rewrite!("external_rule_97";"(or (and (not ?A1) (not ?B1) (not ?C1)) (and (not ?A1) (not ?B1) (not ?C2)) (and (not ?A1) (not ?B2) (not ?C1)) (and (not ?A1) (not ?B2) (not ?C2)) (and (not ?A2) (not ?B1) (not ?C1)) (and (not ?A2) (not ?B1) (not ?C2)) (and (not ?A2) (not ?B2) (not ?C1)) (and (not ?A2) (not ?B2) (not ?C2)) (and (not ?A3) (not ?B1) (not ?C1)) (and (not ?A3) (not ?B1) (not ?C2)) (and (not ?A3) (not ?B2) (not ?C1)) (and (not ?A3) (not ?B2) (not ?C2)))" => "(AOI322xp5_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?B2 ?A2 ?A3)"),
        rewrite!("external_rule_98";"(or (and (not ?A1) (not ?B1)) (and (not ?A1) (not ?B2)) (and (not ?A2) (not ?B1)) (and (not ?A2) (not ?B2)) (and (not ?A3) (not ?B1)) (and (not ?A3) (not ?B2)))" => "(AOI32xp33_ASAP7_75t_L ?A1 ?B1 ?B2 ?A2 ?A3)"),
        rewrite!("external_rule_99";"(or (and (not ?A1) (not ?B1) (not ?C1)) (and (not ?A1) (not ?B2) (not ?C1)) (and (not ?A1) (not ?B3) (not ?C1)) (and (not ?A2) (not ?B1) (not ?C1)) (and (not ?A2) (not ?B2) (not ?C1)) (and (not ?A2) (not ?B3) (not ?C1)) (and (not ?A3) (not ?B1) (not ?C1)) (and (not ?A3) (not ?B2) (not ?C1)) (and (not ?A3) (not ?B3) (not ?C1)))" => "(AOI331xp33_ASAP7_75t_L ?A1 ?B1 ?C1 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_100";"(or (and (not ?A1) (not ?B1) (not ?C1)) (and (not ?A1) (not ?B1) (not ?C2)) (and (not ?A1) (not ?B2) (not ?C1)) (and (not ?A1) (not ?B2) (not ?C2)) (and (not ?A1) (not ?B3) (not ?C1)) (and (not ?A1) (not ?B3) (not ?C2)) (and (not ?A2) (not ?B1) (not ?C1)) (and (not ?A2) (not ?B1) (not ?C2)) (and (not ?A2) (not ?B2) (not ?C1)) (and (not ?A2) (not ?B2) (not ?C2)) (and (not ?A2) (not ?B3) (not ?C1)) (and (not ?A2) (not ?B3) (not ?C2)) (and (not ?A3) (not ?B1) (not ?C1)) (and (not ?A3) (not ?B1) (not ?C2)) (and (not ?A3) (not ?B2) (not ?C1)) (and (not ?A3) (not ?B2) (not ?C2)) (and (not ?A3) (not ?B3) (not ?C1)) (and (not ?A3) (not ?B3) (not ?C2)))" => "(AOI332xp33_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_101";"(or (and (not ?A1) (not ?B1) (not ?C1)) (and (not ?A1) (not ?B1) (not ?C2)) (and (not ?A1) (not ?B1) (not ?C3)) (and (not ?A1) (not ?B2) (not ?C1)) (and (not ?A1) (not ?B2) (not ?C2)) (and (not ?A1) (not ?B2) (not ?C3)) (and (not ?A1) (not ?B3) (not ?C1)) (and (not ?A1) (not ?B3) (not ?C2)) (and (not ?A1) (not ?B3) (not ?C3)) (and (not ?A2) (not ?B1) (not ?C1)) (and (not ?A2) (not ?B1) (not ?C2)) (and (not ?A2) (not ?B1) (not ?C3)) (and (not ?A2) (not ?B2) (not ?C1)) (and (not ?A2) (not ?B2) (not ?C2)) (and (not ?A2) (not ?B2) (not ?C3)) (and (not ?A2) (not ?B3) (not ?C1)) (and (not ?A2) (not ?B3) (not ?C2)) (and (not ?A2) (not ?B3) (not ?C3)) (and (not ?A3) (not ?B1) (not ?C1)) (and (not ?A3) (not ?B1) (not ?C2)) (and (not ?A3) (not ?B1) (not ?C3)) (and (not ?A3) (not ?B2) (not ?C1)) (and (not ?A3) (not ?B2) (not ?C2)) (and (not ?A3) (not ?B2) (not ?C3)) (and (not ?A3) (not ?B3) (not ?C1)) (and (not ?A3) (not ?B3) (not ?C2)) (and (not ?A3) (not ?B3) (not ?C3)))" => "(AOI333xp33_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?C3 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_102";"(or (and (not ?A1) (not ?B1)) (and (not ?A1) (not ?B2)) (and (not ?A1) (not ?B3)) (and (not ?A2) (not ?B1)) (and (not ?A2) (not ?B2)) (and (not ?A2) (not ?B3)) (and (not ?A3) (not ?B1)) (and (not ?A3) (not ?B2)) (and (not ?A3) (not ?B3)))" => "(AOI33xp33_ASAP7_75t_L ?A1 ?B1 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_103";"(or (and ?A1 ?B ?C) (and ?A2 ?B ?C))" => "(OA211x2_ASAP7_75t_L ?A1 ?B ?C ?A2)"),
        rewrite!("external_rule_104";"(or (and ?A1 ?B) (and ?A2 ?B))" => "(OA21x2_ASAP7_75t_L ?A1 ?B ?A2)"),
        rewrite!("external_rule_105";"(or (and ?A1 ?B1 ?C) (and ?A1 ?B2 ?C) (and ?A2 ?B1 ?C) (and ?A2 ?B2 ?C))" => "(OA221x2_ASAP7_75t_L ?A1 ?B1 ?C ?B2 ?A2)"),
        rewrite!("external_rule_106";"(or (and ?A1 ?B1 ?C1) (and ?A1 ?B1 ?C2) (and ?A1 ?B2 ?C1) (and ?A1 ?B2 ?C2) (and ?A2 ?B1 ?C1) (and ?A2 ?B1 ?C2) (and ?A2 ?B2 ?C1) (and ?A2 ?B2 ?C2))" => "(OA222x2_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?B2 ?A2)"),
        rewrite!("external_rule_107";"(or (and ?A1 ?B1) (and ?A1 ?B2) (and ?A2 ?B1) (and ?A2 ?B2))" => "(OA22x2_ASAP7_75t_L ?A1 ?B1 ?B2 ?A2)"),
        rewrite!("external_rule_108";"(or (and ?A1 ?B1) (and ?A2 ?B1) (and ?A3 ?B1))" => "(OA31x2_ASAP7_75t_L ?A1 ?B1 ?A2 ?A3)"),
        rewrite!("external_rule_109";"(or (and ?A1 ?B1 ?C1) (and ?A1 ?B2 ?C1) (and ?A1 ?B3 ?C1) (and ?A2 ?B1 ?C1) (and ?A2 ?B2 ?C1) (and ?A2 ?B3 ?C1) (and ?A3 ?B1 ?C1) (and ?A3 ?B2 ?C1) (and ?A3 ?B3 ?C1))" => "(OA331x1_ASAP7_75t_L ?A1 ?B1 ?C1 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_110";"(or (and ?A1 ?B1 ?C1) (and ?A1 ?B2 ?C1) (and ?A1 ?B3 ?C1) (and ?A2 ?B1 ?C1) (and ?A2 ?B2 ?C1) (and ?A2 ?B3 ?C1) (and ?A3 ?B1 ?C1) (and ?A3 ?B2 ?C1) (and ?A3 ?B3 ?C1))" => "(OA331x2_ASAP7_75t_L ?A1 ?B1 ?C1 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_111";"(or (and ?A1 ?B1 ?C1) (and ?A1 ?B1 ?C2) (and ?A1 ?B2 ?C1) (and ?A1 ?B2 ?C2) (and ?A1 ?B3 ?C1) (and ?A1 ?B3 ?C2) (and ?A2 ?B1 ?C1) (and ?A2 ?B1 ?C2) (and ?A2 ?B2 ?C1) (and ?A2 ?B2 ?C2) (and ?A2 ?B3 ?C1) (and ?A2 ?B3 ?C2) (and ?A3 ?B1 ?C1) (and ?A3 ?B1 ?C2) (and ?A3 ?B2 ?C1) (and ?A3 ?B2 ?C2) (and ?A3 ?B3 ?C1) (and ?A3 ?B3 ?C2))" => "(OA332x1_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_112";"(or (and ?A1 ?B1 ?C1) (and ?A1 ?B1 ?C2) (and ?A1 ?B2 ?C1) (and ?A1 ?B2 ?C2) (and ?A1 ?B3 ?C1) (and ?A1 ?B3 ?C2) (and ?A2 ?B1 ?C1) (and ?A2 ?B1 ?C2) (and ?A2 ?B2 ?C1) (and ?A2 ?B2 ?C2) (and ?A2 ?B3 ?C1) (and ?A2 ?B3 ?C2) (and ?A3 ?B1 ?C1) (and ?A3 ?B1 ?C2) (and ?A3 ?B2 ?C1) (and ?A3 ?B2 ?C2) (and ?A3 ?B3 ?C1) (and ?A3 ?B3 ?C2))" => "(OA332x2_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_113";"(or (and ?A1 ?B1 ?C1) (and ?A1 ?B1 ?C2) (and ?A1 ?B1 ?C3) (and ?A1 ?B2 ?C1) (and ?A1 ?B2 ?C2) (and ?A1 ?B2 ?C3) (and ?A1 ?B3 ?C1) (and ?A1 ?B3 ?C2) (and ?A1 ?B3 ?C3) (and ?A2 ?B1 ?C1) (and ?A2 ?B1 ?C2) (and ?A2 ?B1 ?C3) (and ?A2 ?B2 ?C1) (and ?A2 ?B2 ?C2) (and ?A2 ?B2 ?C3) (and ?A2 ?B3 ?C1) (and ?A2 ?B3 ?C2) (and ?A2 ?B3 ?C3) (and ?A3 ?B1 ?C1) (and ?A3 ?B1 ?C2) (and ?A3 ?B1 ?C3) (and ?A3 ?B2 ?C1) (and ?A3 ?B2 ?C2) (and ?A3 ?B2 ?C3) (and ?A3 ?B3 ?C1) (and ?A3 ?B3 ?C2) (and ?A3 ?B3 ?C3))" => "(OA333x1_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?C3 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_114";"(or (and ?A1 ?B1 ?C1) (and ?A1 ?B1 ?C2) (and ?A1 ?B1 ?C3) (and ?A1 ?B2 ?C1) (and ?A1 ?B2 ?C2) (and ?A1 ?B2 ?C3) (and ?A1 ?B3 ?C1) (and ?A1 ?B3 ?C2) (and ?A1 ?B3 ?C3) (and ?A2 ?B1 ?C1) (and ?A2 ?B1 ?C2) (and ?A2 ?B1 ?C3) (and ?A2 ?B2 ?C1) (and ?A2 ?B2 ?C2) (and ?A2 ?B2 ?C3) (and ?A2 ?B3 ?C1) (and ?A2 ?B3 ?C2) (and ?A2 ?B3 ?C3) (and ?A3 ?B1 ?C1) (and ?A3 ?B1 ?C2) (and ?A3 ?B1 ?C3) (and ?A3 ?B2 ?C1) (and ?A3 ?B2 ?C2) (and ?A3 ?B2 ?C3) (and ?A3 ?B3 ?C1) (and ?A3 ?B3 ?C2) (and ?A3 ?B3 ?C3))" => "(OA333x2_ASAP7_75t_L ?A1 ?B1 ?C1 ?C2 ?C3 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_115";"(or (and ?A1 ?B1) (and ?A1 ?B2) (and ?A1 ?B3) (and ?A2 ?B1) (and ?A2 ?B2) (and ?A2 ?B3) (and ?A3 ?B1) (and ?A3 ?B2) (and ?A3 ?B3))" => "(OA33x2_ASAP7_75t_L ?A1 ?B1 ?B2 ?B3 ?A2 ?A3)"),
        rewrite!("external_rule_116";"(or (and (not ?A1) (not ?A2)) (not ?B) (not ?C))" => "(OAI211xp5_ASAP7_75t_L ?A1 ?A2 ?B ?C)"),
        rewrite!("external_rule_117";"(or (and (not ?A1) (not ?A2)) (not ?B))" => "(OAI21x1_ASAP7_75t_L ?A1 ?A2 ?B)"),
        rewrite!("external_rule_118";"(or (and (not ?A1) (not ?A2)) (not ?B))" => "(OAI21xp33_ASAP7_75t_L ?A1 ?A2 ?B)"),
        rewrite!("external_rule_119";"(or (and (not ?A1) (not ?A2)) (not ?B))" => "(OAI21xp5_ASAP7_75t_L ?A1 ?A2 ?B)"),
        rewrite!("external_rule_120";"(or (and (not ?A1) (not ?A2)) (and (not ?B1) (not ?B2)) (not ?C))" => "(OAI221xp5_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2 ?C)"),
        rewrite!("external_rule_121";"(or (and (not ?A1) (not ?A2)) (and (not ?B1) (not ?B2)) (and (not ?C1) (not ?C2)))" => "(OAI222xp33_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2 ?C1 ?C2)"),
        rewrite!("external_rule_122";"(or (and (not ?A1) (not ?A2)) (and (not ?B1) (not ?B2)))" => "(OAI22x1_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2)"),
        rewrite!("external_rule_123";"(or (and (not ?A1) (not ?A2)) (and (not ?B1) (not ?B2)))" => "(OAI22xp33_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2)"),
        rewrite!("external_rule_124";"(or (and (not ?A1) (not ?A2)) (and (not ?B1) (not ?B2)))" => "(OAI22xp5_ASAP7_75t_L ?A1 ?A2 ?B1 ?B2)"),
        rewrite!("external_rule_125";"(or (and (not ?A1) (not ?A2) (not ?A3)) (not ?B1) (not ?C1))" => "(OAI311xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?C1)"),
        rewrite!("external_rule_126";"(or (and (not ?A1) (not ?A2) (not ?A3)) (not ?B))" => "(OAI31xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B)"),
        rewrite!("external_rule_127";"(or (and (not ?A1) (not ?A2) (not ?A3)) (not ?B))" => "(OAI31xp67_ASAP7_75t_L ?A1 ?A2 ?A3 ?B)"),
        rewrite!("external_rule_128";"(or (and (not ?A1) (not ?A2) (not ?A3)) (and (not ?B1) (not ?B2)) (not ?C))" => "(OAI321xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?C)"),
        rewrite!("external_rule_129";"(or (and (not ?A1) (not ?A2) (not ?A3)) (and (not ?B1) (not ?B2)) (and (not ?C1) (not ?C2)))" => "(OAI322xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?C1 ?C2)"),
        rewrite!("external_rule_130";"(or (and (not ?A1) (not ?A2) (not ?A3)) (and (not ?B1) (not ?B2)))" => "(OAI32xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2)"),
        rewrite!("external_rule_131";"(or (and (not ?A1) (not ?A2) (not ?A3)) (and (not ?B1) (not ?B2) (not ?B3)) (not ?C1))" => "(OAI331xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C1)"),
        rewrite!("external_rule_132";"(or (and (not ?A1) (not ?A2) (not ?A3)) (and (not ?B1) (not ?B2) (not ?B3)) (and (not ?C1) (not ?C2)))" => "(OAI332xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C1 ?C2)"),
        rewrite!("external_rule_133";"(or (and (not ?A1) (not ?A2) (not ?A3)) (and (not ?B1) (not ?B2) (not ?B3)) (and (not ?C1) (not ?C2) (not ?C3)))" => "(OAI333xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C1 ?C2 ?C3)"),
        rewrite!("external_rule_134";"(or (and (not ?A1) (not ?A2) (not ?A3)) (and (not ?B1) (not ?B2) (not ?B3)) (and (not ?C1) (not ?C2) (not ?C3)))" => "(OAI33xp33_ASAP7_75t_L ?A1 ?A2 ?A3 ?B1 ?B2 ?B3 ?C1 ?C2 ?C3)"),
    ];

    rules
}
