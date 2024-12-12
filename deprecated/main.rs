mod lib;
mod dynamic_rules;
mod static_rules;
mod lp;
use egg::*;
use egraph_mapping::{SimpleLanguage, convert_to_simple_language_enum, get_node_type};
use std::collections::HashMap;
use std::time::Instant;
use std::time::Duration;
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashSet;


// #[derive(Debug)]
// pub struct CombinedCost;

// impl CostFunction<SimpleLanguage> for CombinedCost {
//     type Cost = (f64, f64, f64);

//     fn cost<C>(&mut self, enode: &SimpleLanguage, mut cost: C) -> Self::Cost
//     where
//         C: FnMut(Id) -> Self::Cost,
//     {
//         let cells = lib::parse_genlib("7nm.genlib");

//         // Helper function to get the node type as a string

//         let (node_size, node_depth) = match enode {
//             SimpleLanguage::Symbol(_) => (0.0, 0.0),
//             _ => {
//                 let node_type = get_node_type(enode);
//                 cells.iter()
//                     .find(|&&(ref name, _, _, _)| name == node_type)
//                     .map_or((5000.0, 5000.0), |&(_, size, depth, _)| (size, depth))
//             }
//         };

//         let ast_size = node_size + enode.fold(0.0_f64, |sum, id| sum + cost(id).1);
//         let ast_depth = node_depth + enode.fold(0.0_f64, |max, id| max.max(cost(id).2));
//         (ast_size, ast_size, ast_depth)
//     }
// }

// pub struct Modified_AstSizeCost{
//     pub counted: Vec<Id>,
// }

// impl CostFunction<SimpleLanguage> for Modified_AstSizeCost {
//     type Cost = usize;
//     fn cost<C>(&mut self, enode: &SimpleLanguage, mut costs: C) -> Self::Cost
//     where
//         C: FnMut(Id) -> Self::Cost,
//     {
//         let mut sum = 1;
        
//         for id in enode.children() {

//             if !self.counted.contains(id){
//                 sum += costs(*id);
//                 self.counted.push(*id);
//             }
//         }

//         sum
//     }
// }

pub struct WeightedAstSize{
    time1: f64,
    time2: f64
}

impl CostFunction<SimpleLanguage> for WeightedAstSize {
    type Cost = (usize,usize,Vec<usize>);
    fn cost<C>(&mut self, enode: &SimpleLanguage, mut costs: C) -> Self::Cost
    where
        C: FnMut(Id) -> Self::Cost,
    {
        let cells = [("_const0_", 0, 100, "z=CONST0"), ("_const1_", 0, 100, "z=CONST1"), ("BUFx10_ASAP7_75t_L", 327, 100, "Y=A"), ("BUFx12_ASAP7_75t_L", 373, 100, "Y=A"), ("BUFx12f_ASAP7_75t_L", 420, 100, "Y=A"), ("BUFx16f_ASAP7_75t_L", 513, 100, "Y=A"), ("BUFx24_ASAP7_75t_L", 700, 100, "Y=A"), ("BUFx2_ASAP7_75t_L", 117, 100, "Y=A"), ("BUFx3_ASAP7_75t_L", 140, 100, "Y=A"), ("BUFx4_ASAP7_75t_L", 163, 100, "Y=A"), ("BUFx4f_ASAP7_75t_L", 187, 100, "Y=A"), ("BUFx5_ASAP7_75t_L", 187, 100, "Y=A"), ("BUFx6f_ASAP7_75t_L", 233, 100, "Y=A"), ("BUFx8_ASAP7_75t_L", 280, 100, "Y=A"), ("HB1xp67_ASAP7_75t_L", 93, 100, "Y=A"), ("HB2xp67_ASAP7_75t_L", 117, 100, "Y=A"), ("HB3xp67_ASAP7_75t_L", 140, 100, "Y=A"), ("HB4xp67_ASAP7_75t_L", 163, 100, "Y=A"), ("INVx11_ASAP7_75t_L", 303, 100, "Y=!A"), ("INVx13_ASAP7_75t_L", 350, 100, "Y=!A"), ("INVx1_ASAP7_75t_L", 70, 100, "Y=!A"), ("INVx2_ASAP7_75t_L", 93, 100, "Y=!A"), ("INVx3_ASAP7_75t_L", 117, 100, "Y=!A"), ("INVx4_ASAP7_75t_L", 140, 100, "Y=!A"), ("INVx5_ASAP7_75t_L", 163, 100, "Y=!A"), ("INVx6_ASAP7_75t_L", 187, 100, "Y=!A"), ("INVx8_ASAP7_75t_L", 233, 100, "Y=!A"), ("INVxp33_ASAP7_75t_L", 70, 100, "Y=!A"), ("INVxp67_ASAP7_75t_L", 70, 100, "Y=!A"), ("AND2x2_ASAP7_75t_L", 140, 100, "Y=(A * B)"), ("AND2x4_ASAP7_75t_L", 233, 100, "Y=(A * B)"), ("AND2x6_ASAP7_75t_L", 280, 100, "Y=(A * B)"), ("AND3x1_ASAP7_75t_L", 140, 100, "Y=(A * B * C)"), ("AND3x2_ASAP7_75t_L", 163, 100, "Y=(A * B * C)"), ("AND3x4_ASAP7_75t_L", 373, 100, "Y=(A * B * C)"), ("AND4x1_ASAP7_75t_L", 163, 100, "Y=(A * B * C * D)"), ("AND4x2_ASAP7_75t_L", 373, 100, "Y=(A * B * C * D)"), ("AND5x1_ASAP7_75t_L", 187, 100, "Y=(A * B * C * D * E)"), ("AND5x2_ASAP7_75t_L", 467, 100, "Y=(A * B * C * D * E)"), ("MAJIxp5_ASAP7_75t_L", 163, 100, "Y=(!A * !B) + (!A * !C) + (!B * !C)"), ("MAJx2_ASAP7_75t_L", 210, 100, "Y=(A * B) + (A * C) + (B * C)"), ("MAJx3_ASAP7_75t_L", 233, 100, "Y=(A * B) + (A * C) + (B * C)"), ("NAND2x1_ASAP7_75t_L", 140, 100, "Y=(!A) + (!B)"), ("NAND2x1p5_ASAP7_75t_L", 187, 100, "Y=(!A) + (!B)"), ("NAND2x2_ASAP7_75t_L", 233, 100, "Y=(!A) + (!B)"), ("NAND2xp33_ASAP7_75t_L", 93, 100, "Y=(!A) + (!B)"), ("NAND2xp5_ASAP7_75t_L", 93, 100, "Y=(!A) + (!B)"), ("NAND2xp67_ASAP7_75t_L", 140, 100, "Y=(!A) + (!B)"), ("NAND3x1_ASAP7_75t_L", 257, 100, "Y=(!A) + (!B) + (!C)"), ("NAND3x2_ASAP7_75t_L", 467, 100, "Y=(!A) + (!B) + (!C)"), ("NAND3xp33_ASAP7_75t_L", 117, 100, "Y=(!A) + (!B) + (!C)"), ("NAND4xp25_ASAP7_75t_L", 140, 100, "Y=(!A) + (!B) + (!C) + (!D)"), ("NAND4xp75_ASAP7_75t_L", 327, 100, "Y=(!A) + (!B) + (!C) + (!D)"), ("NAND5xp2_ASAP7_75t_L", 163, 100, "Y=(!A) + (!B) + (!C) + (!D) + (!E)"), ("NOR2x1_ASAP7_75t_L", 140, 100, "Y=(!A * !B)"), ("NOR2x1p5_ASAP7_75t_L", 187, 100, "Y=(!A * !B)"), ("NOR2x2_ASAP7_75t_L", 233, 100, "Y=(!A * !B)"), ("NOR2xp33_ASAP7_75t_L", 93, 100, "Y=(!A * !B)"), ("NOR2xp67_ASAP7_75t_L", 140, 100, "Y=(!A * !B)"), ("NOR3x1_ASAP7_75t_L", 257, 100, "Y=(!A * !B * !C)"), ("NOR3x2_ASAP7_75t_L", 467, 100, "Y=(!A * !B * !C)"), ("NOR3xp33_ASAP7_75t_L", 117, 100, "Y=(!A * !B * !C)"), ("NOR4xp25_ASAP7_75t_L", 140, 100, "Y=(!A * !B * !C * !D)"), ("NOR4xp75_ASAP7_75t_L", 327, 100, "Y=(!A * !B * !C * !D)"), ("NOR5xp2_ASAP7_75t_L", 163, 100, "Y=(!A * !B * !C * !D * !E)"), ("OR2x2_ASAP7_75t_L", 140, 100, "Y=(A) + (B)"), ("OR2x4_ASAP7_75t_L", 187, 100, "Y=(A) + (B)"), ("OR2x6_ASAP7_75t_L", 280, 100, "Y=(A) + (B)"), ("OR3x1_ASAP7_75t_L", 140, 100, "Y=(A) + (B) + (C)"), ("OR3x2_ASAP7_75t_L", 163, 100, "Y=(A) + (B) + (C)"), ("OR3x4_ASAP7_75t_L", 210, 100, "Y=(A) + (B) + (C)"), ("OR4x1_ASAP7_75t_L", 163, 100, "Y=(A) + (B) + (C) + (D)"), ("OR4x2_ASAP7_75t_L", 187, 100, "Y=(A) + (B) + (C) + (D)"), ("OR5x1_ASAP7_75t_L", 187, 100, "Y=(A) + (B) + (C) + (D) + (E)"), ("OR5x2_ASAP7_75t_L", 210, 100, "Y=(A) + (B) + (C) + (D) + (E)"), ("XNOR2x1_ASAP7_75t_L", 280, 100, "Y=(A * B) + (!A * !B)"), ("XNOR2x2_ASAP7_75t_L", 257, 100, "Y=(A * B) + (!A * !B)"), ("XNOR2xp5_ASAP7_75t_L", 210, 100, "Y=(A * B) + (!A * !B)"), ("XOR2x1_ASAP7_75t_L", 280, 100, "Y=(A * !B) + (!A * B)"), ("XOR2x2_ASAP7_75t_L", 257, 100, "Y=(A * !B) + (!A * B)"), ("XOR2xp5_ASAP7_75t_L", 210, 100, "Y=(A * !B) + (!A * B)"), ("A2O1A1Ixp33_ASAP7_75t_L", 260, 100, "Y=(!A1 * !B) + (!A2 * !B) + (!C)"), ("A2O1A1O1Ixp25_ASAP7_75t_L", 400, 100, "Y=(!A1 * !B * !D) + (!A2 * !B * !D) + (!C * !D)"), ("AO211x2_ASAP7_75t_L", 373, 100, "Y=(A1 * A2) + (B) + (C)"), ("AO21x1_ASAP7_75t_L", 140, 100, "Y=(A1 * A2) + (B)"), ("AO21x2_ASAP7_75t_L", 163, 100, "Y=(A1 * A2) + (B)"), ("AO221x1_ASAP7_75t_L", 233, 100, "Y=(A1 * A2) + (B1 * B2) + (C)"), ("AO221x2_ASAP7_75t_L", 257, 100, "Y=(A1 * A2) + (B1 * B2) + (C)"), ("AO222x2_ASAP7_75t_L", 513, 100, "Y=(A1 * A2) + (B1 * B2) + (C1 * C2)"), ("AO22x1_ASAP7_75t_L", 210, 100, "Y=(A1 * A2) + (B1 * B2)"), ("AO22x2_ASAP7_75t_L", 233, 100, "Y=(A1 * A2) + (B1 * B2)"), ("AO31x2_ASAP7_75t_L", 373, 100, "Y=(A1 * A2 * A3) + (B)"), ("AO322x2_ASAP7_75t_L", 350, 100, "Y=(A1 * A2 * A3) + (B1 * B2) + (C1 * C2)"), ("AO32x1_ASAP7_75t_L", 187, 100, "Y=(A1 * A2 * A3) + (B1 * B2)"), ("AO32x2_ASAP7_75t_L", 210, 100, "Y=(A1 * A2 * A3) + (B1 * B2)"), ("AO331x1_ASAP7_75t_L", 233, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C)"), ("AO331x2_ASAP7_75t_L", 257, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C)"), ("AO332x1_ASAP7_75t_L", 257, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2)"), ("AO332x2_ASAP7_75t_L", 280, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2)"), ("AO333x1_ASAP7_75t_L", 280, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2 * C3)"), ("AO333x2_ASAP7_75t_L", 303, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3) + (C1 * C2 * C3)"), ("AO33x2_ASAP7_75t_L", 233, 100, "Y=(A1 * A2 * A3) + (B1 * B2 * B3)"), ("AOI211x1_ASAP7_75t_L", 280, 100, "Y=(!A1 * !B * !C) + (!A2 * !B * !C)"), ("AOI211xp5_ASAP7_75t_L", 140, 100, "Y=(!A1 * !B * !C) + (!A2 * !B * !C)"), ("AOI21x1_ASAP7_75t_L", 187, 100, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI21xp33_ASAP7_75t_L", 117, 100, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI21xp5_ASAP7_75t_L", 117, 100, "Y=(!A1 * !B) + (!A2 * !B)"), ("AOI221x1_ASAP7_75t_L", 327, 100, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C)"), ("AOI221xp5_ASAP7_75t_L", 163, 100, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C)"), ("AOI222xp33_ASAP7_75t_L", 233, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2)"), ("AOI22x1_ASAP7_75t_L", 233, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI22xp33_ASAP7_75t_L", 140, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI22xp5_ASAP7_75t_L", 140, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2)"), ("AOI311xp33_ASAP7_75t_L", 163, 100, "Y=(!A1 * !B * !C) + (!A2 * !B * !C) + (!A3 * !B * !C)"), ("AOI31xp33_ASAP7_75t_L", 140, 100, "Y=(!A1 * !B) + (!A2 * !B) + (!A3 * !B)"), ("AOI31xp67_ASAP7_75t_L", 303, 100, "Y=(!A1 * !B) + (!A2 * !B) + (!A3 * !B)"), ("AOI321xp33_ASAP7_75t_L", 187, 100, "Y=(!A1 * !B1 * !C) + (!A1 * !B2 * !C) + (!A2 * !B1 * !C) + (!A2 * !B2 * !C) + (!A3 * !B1 * !C) + (!A3 * !B2 * !C)"), ("AOI322xp5_ASAP7_75t_L", 210, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2)"), ("AOI32xp33_ASAP7_75t_L", 163, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A2 * !B1) + (!A2 * !B2) + (!A3 * !B1) + (!A3 * !B2)"), ("AOI331xp33_ASAP7_75t_L", 210, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B2 * !C1) + (!A1 * !B3 * !C1) + (!A2 * !B1 * !C1) + (!A2 * !B2 * !C1) + (!A2 * !B3 * !C1) + (!A3 * !B1 * !C1) + (!A3 * !B2 * !C1) + (!A3 * !B3 * !C1)"), ("AOI332xp33_ASAP7_75t_L", 233, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A1 * !B3 * !C1) + (!A1 * !B3 * !C2) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A2 * !B3 * !C1) + (!A2 * !B3 * !C2) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2) + (!A3 * !B3 * !C1) + (!A3 * !B3 * !C2)"), ("AOI333xp33_ASAP7_75t_L", 257, 100, "Y=(!A1 * !B1 * !C1) + (!A1 * !B1 * !C2) + (!A1 * !B1 * !C3) + (!A1 * !B2 * !C1) + (!A1 * !B2 * !C2) + (!A1 * !B2 * !C3) + (!A1 * !B3 * !C1) + (!A1 * !B3 * !C2) + (!A1 * !B3 * !C3) + (!A2 * !B1 * !C1) + (!A2 * !B1 * !C2) + (!A2 * !B1 * !C3) + (!A2 * !B2 * !C1) + (!A2 * !B2 * !C2) + (!A2 * !B2 * !C3) + (!A2 * !B3 * !C1) + (!A2 * !B3 * !C2) + (!A2 * !B3 * !C3) + (!A3 * !B1 * !C1) + (!A3 * !B1 * !C2) + (!A3 * !B1 * !C3) + (!A3 * !B2 * !C1) + (!A3 * !B2 * !C2) + (!A3 * !B2 * !C3) + (!A3 * !B3 * !C1) + (!A3 * !B3 * !C2) + (!A3 * !B3 * !C3)"), ("AOI33xp33_ASAP7_75t_L", 187, 100, "Y=(!A1 * !B1) + (!A1 * !B2) + (!A1 * !B3) + (!A2 * !B1) + (!A2 * !B2) + (!A2 * !B3) + (!A3 * !B1) + (!A3 * !B2) + (!A3 * !B3)"), ("O2A1O1Ixp33_ASAP7_75t_L", 0, 100, "Y=(!A1 * !A2 * !C) + (!B * !C)"), ("O2A1O1Ixp5_ASAP7_75t_L", 0, 100, "Y=(!A1 * !A2 * !C) + (!B * !C)"), ("OA211x2_ASAP7_75t_L", 187, 100, "Y=(A1 * B * C) + (A2 * B * C)"), ("OA21x2_ASAP7_75t_L", 163, 100, "Y=(A1 * B) + (A2 * B)"), ("OA221x2_ASAP7_75t_L", 373, 100, "Y=(A1 * B1 * C) + (A1 * B2 * C) + (A2 * B1 * C) + (A2 * B2 * C)"), ("OA222x2_ASAP7_75t_L", 280, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2)"), ("OA22x2_ASAP7_75t_L", 233, 100, "Y=(A1 * B1) + (A1 * B2) + (A2 * B1) + (A2 * B2)"), ("OA31x2_ASAP7_75t_L", 350, 100, "Y=(A1 * B1) + (A2 * B1) + (A3 * B1)"), ("OA331x1_ASAP7_75t_L", 233, 100, "Y=(A1 * B1 * C1) + (A1 * B2 * C1) + (A1 * B3 * C1) + (A2 * B1 * C1) + (A2 * B2 * C1) + (A2 * B3 * C1) + (A3 * B1 * C1) + (A3 * B2 * C1) + (A3 * B3 * C1)"), ("OA331x2_ASAP7_75t_L", 257, 100, "Y=(A1 * B1 * C1) + (A1 * B2 * C1) + (A1 * B3 * C1) + (A2 * B1 * C1) + (A2 * B2 * C1) + (A2 * B3 * C1) + (A3 * B1 * C1) + (A3 * B2 * C1) + (A3 * B3 * C1)"), ("OA332x1_ASAP7_75t_L", 257, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B3 * C1) + (A3 * B3 * C2)"), ("OA332x2_ASAP7_75t_L", 280, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B3 * C1) + (A3 * B3 * C2)"), ("OA333x1_ASAP7_75t_L", 280, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B1 * C3) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B2 * C3) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A1 * B3 * C3) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B1 * C3) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B2 * C3) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A2 * B3 * C3) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B1 * C3) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B2 * C3) + (A3 * B3 * C1) + (A3 * B3 * C2) + (A3 * B3 * C3)"), ("OA333x2_ASAP7_75t_L", 303, 100, "Y=(A1 * B1 * C1) + (A1 * B1 * C2) + (A1 * B1 * C3) + (A1 * B2 * C1) + (A1 * B2 * C2) + (A1 * B2 * C3) + (A1 * B3 * C1) + (A1 * B3 * C2) + (A1 * B3 * C3) + (A2 * B1 * C1) + (A2 * B1 * C2) + (A2 * B1 * C3) + (A2 * B2 * C1) + (A2 * B2 * C2) + (A2 * B2 * C3) + (A2 * B3 * C1) + (A2 * B3 * C2) + (A2 * B3 * C3) + (A3 * B1 * C1) + (A3 * B1 * C2) + (A3 * B1 * C3) + (A3 * B2 * C1) + (A3 * B2 * C2) + (A3 * B2 * C3) + (A3 * B3 * C1) + (A3 * B3 * C2) + (A3 * B3 * C3)"), ("OA33x2_ASAP7_75t_L", 233, 100, "Y=(A1 * B1) + (A1 * B2) + (A1 * B3) + (A2 * B1) + (A2 * B2) + (A2 * B3) + (A3 * B1) + (A3 * B2) + (A3 * B3)"), ("OAI211xp5_ASAP7_75t_L", 140, 100, "Y=(!A1 * !A2) + (!B) + (!C)"), ("OAI21x1_ASAP7_75t_L", 187, 100, "Y=(!A1 * !A2) + (!B)"), ("OAI21xp33_ASAP7_75t_L", 117, 100, "Y=(!A1 * !A2) + (!B)"), ("OAI21xp5_ASAP7_75t_L", 117, 100, "Y=(!A1 * !A2) + (!B)"), ("OAI221xp5_ASAP7_75t_L", 163, 100, "Y=(!A1 * !A2) + (!B1 * !B2) + (!C)"), ("OAI222xp33_ASAP7_75t_L", 233, 100, "Y=(!A1 * !A2) + (!B1 * !B2) + (!C1 * !C2)"), ("OAI22x1_ASAP7_75t_L", 233, 100, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI22xp33_ASAP7_75t_L", 140, 100, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI22xp5_ASAP7_75t_L", 140, 100, "Y=(!A1 * !A2) + (!B1 * !B2)"), ("OAI311xp33_ASAP7_75t_L", 163, 100, "Y=(!A1 * !A2 * !A3) + (!B1) + (!C1)"), ("OAI31xp33_ASAP7_75t_L", 140, 100, "Y=(!A1 * !A2 * !A3) + (!B)"), ("OAI31xp67_ASAP7_75t_L", 303, 100, "Y=(!A1 * !A2 * !A3) + (!B)"), ("OAI321xp33_ASAP7_75t_L", 187, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2) + (!C)"), ("OAI322xp33_ASAP7_75t_L", 210, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2) + (!C1 * !C2)"), ("OAI32xp33_ASAP7_75t_L", 163, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2)"), ("OAI331xp33_ASAP7_75t_L", 210, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1)"), ("OAI332xp33_ASAP7_75t_L", 233, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2)"), ("OAI333xp33_ASAP7_75t_L", 257, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2 * !C3)"), ("OAI33xp33_ASAP7_75t_L", 257, 100, "Y=(!A1 * !A2 * !A3) + (!B1 * !B2 * !B3) + (!C1 * !C2 * !C3)")];


        fn insert_sorted(vec: &mut Vec<usize>, value: usize, duplicates: &mut HashMap<usize, usize>) {
            match vec.binary_search(&value) {
                Ok(_) => {
                    // 如果找到相同的值，增加计数，如果不存在则初始化为 2
                    *duplicates.entry(value).or_insert(1) += 1;
                },
                Err(pos) => {
                    // 如果没找到，插入该值到适当位置
                    vec.insert(pos, value);
                },
            }
        }

        // let start1 = Instant::now();

        let (mut unit, _) = match enode {
            SimpleLanguage::Symbol(_) => (0, 0),
            SimpleLanguage::OUT(_) => (0, 0),
            _ => {
                let node_type = &get_node_type(enode);
                cells.iter()
                    .find(|&&(ref name, _, _, _)| name == node_type)
                    .map_or((500000, 500000), |&(_, size, depth, _)| (size, depth))
            }
        };
        

        let mut node_size = unit;

        let start2 = Instant::now();

        let mut combined: Vec<usize> = Vec::new();
        let mut duplicates: HashMap<usize, usize> = HashMap::new(); // 用于存储重复元素和其出现次数
        let children_set: HashSet<egg::Id> = enode.children().iter().cloned().collect();
        println!("---------------");
        for id in children_set {
            node_size += costs(id).0;
            let mut current_slice = costs(id).2.clone(); // 当前子节点的成本数据，已排序

            insert_sorted(&mut combined, usize::from(id), & mut duplicates);

            if combined.is_empty() {
                combined = current_slice;
                continue;
            }

            if combined.first()>current_slice.last(){
                current_slice.append(&mut combined);
                combined = current_slice;
            }
            else if current_slice.first()>combined.last() {
                combined.append(&mut current_slice);
            }
            else if current_slice.first() >= combined.first() && current_slice.last() <= combined.last(){
                // let mut overlap_current = current_slice;
                let first_possible = match current_slice.first().and_then(|&first| combined.binary_search(&first).ok()) {
                    Some(index) => index,
                    None => 0, // 没有重叠的开始
                };
                let last_possible = match current_slice.last().and_then(|&last| Some(combined.binary_search(&last))) {
                    Some(Ok(index)) => index+1,  // 找到元素，返回Some(index)
                    Some(Err(index)) => index,
                    None => panic!(),
                };
                let (mut merged_vec, counts) = lib::merge_sorted_vecs_unique(current_slice, combined[first_possible..last_possible].to_vec());
                lib::merge_and_update_maps(&mut duplicates, &counts);
                let mut new_combined = combined[0..first_possible].to_vec();
                new_combined.append(&mut merged_vec);
                new_combined.append(&mut combined[last_possible..].to_vec());
                combined=new_combined;
            }
            else if combined.first() >= current_slice.first() && combined.last() <= current_slice.last(){
                let first_possible = match combined.first().and_then(|&first| current_slice.binary_search(&first).ok()) {
                    Some(index) => index,
                    None => 0,
                };
                let last_possible = match combined.last().and_then(|&last| Some(current_slice.binary_search(&last))) {
                    Some(Ok(index)) => index+1,  // 找到元素，返回Some(index)
                    Some(Err(index)) => index,
                    None => panic!(),
                };
                let (mut merged_vec, counts) = lib::merge_sorted_vecs_unique(combined.clone(), current_slice[first_possible..last_possible].to_vec());
                lib::merge_and_update_maps(&mut duplicates, &counts);
                let mut new_combined = current_slice[0..first_possible].to_vec();
                new_combined.append(&mut merged_vec);
                new_combined.append(&mut current_slice[last_possible..].to_vec());
                combined=new_combined;
            }
            else if current_slice.first() >= combined.first() && current_slice.last() >= combined.last(){
                let first_possible = match current_slice.first().and_then(|&first| combined.binary_search(&first).ok()) {
                    Some(index) => index,
                    None => 0,
                };
                let overlapped_combine=combined[first_possible..].to_vec();
                let last_possible = match combined.last().and_then(|&last| Some(current_slice.binary_search(&last))) {
                    Some(Ok(index)) => index+1,  // 找到元素，返回Some(index)
                    Some(Err(index)) => index,
                    None => panic!(),
                };
                let overlapped_current=current_slice[..last_possible].to_vec();
                let (mut merged_vec, counts) = lib::merge_sorted_vecs_unique(overlapped_combine.clone(), overlapped_current.clone());
                lib::merge_and_update_maps(&mut duplicates, &counts);
                let mut new_combined = combined[..first_possible].to_vec();
                new_combined.append(&mut merged_vec);
                new_combined.append(&mut current_slice[last_possible..].to_vec());
                combined=new_combined;
            }
            else if combined.first() >= current_slice.first() && combined.last() >= current_slice.last(){
                let first_possible = match combined.first().and_then(|&first| current_slice.binary_search(&first).ok()) {
                    Some(index) => index,
                    None => 0,
                };
                let overlapped_current=current_slice[first_possible..].to_vec();
                let last_possible = match current_slice.last().and_then(|&last| Some(combined.binary_search(&last))) {
                    Some(Ok(index)) => index+1,  // 找到元素，返回Some(index)
                    Some(Err(index)) => index,
                    None => panic!(),
                };
                let overlapped_combine=combined[..last_possible].to_vec();
                let (mut merged_vec, counts) = lib::merge_sorted_vecs_unique(overlapped_combine.clone(), overlapped_current.clone());
                lib::merge_and_update_maps(&mut duplicates, &counts);
                let mut new_combined = current_slice[..first_possible].to_vec();
                new_combined.append(&mut merged_vec);
                new_combined.append(&mut combined[last_possible..].to_vec());
                combined=new_combined;
            }
        }

        for (&element, &count) in duplicates.iter() {
            node_size -= (count - 1) * costs(Id::from(element)).1 as usize;
        }

        let duration2 = start2.elapsed();
        let time2 = duration2.as_secs() as f64 + duration2.subsec_nanos() as f64 * 1e-9;
        self.time2=self.time2+time2;

        let start3 = Instant::now();
        let mut node_size1 = unit;
        let mut combined1:Vec<usize> = Vec::new();
        for id in enode.children() {
            node_size1 += costs(*id).0;
            combined1.push(usize::from(*id));
            combined1.extend(costs(*id).2.clone());
        }


        // println!("    enode size:{}",unit);

        let mut counts1 = HashMap::new();
        for value in &combined1 {
            *counts1.entry(*value).or_insert(0) += 1;
        }


        let set1: HashSet<_> = combined1.clone().into_iter().collect();
        let mut unique_elements1: Vec<_> = set1.into_iter().collect();
        let duplicates1: HashMap<_, _> = counts1.iter()
        .filter(|&(_, &count)| count > 1)
        .map(|(&k, &v)| (k, v))
        .collect();

        for (&element, &count) in duplicates1.iter() {
            node_size1 -= (count - 1) * costs(Id::from(element)).1 as usize;
        }

        let duration3 = start3.elapsed();
        let time3 = duration3.as_secs() as f64 + duration3.subsec_nanos() as f64 * 1e-9;
        self.time1=self.time1+time3;

        unique_elements1.sort();

        if unique_elements1!=combined {
            println!("{:?},{:?}",unique_elements1,combined);
            panic!("error4");
        }

        if node_size1!=node_size{
            println!("{:?},{:?}",duplicates1,duplicates);
            println!("{:?},{:?}",node_size1,node_size);
            panic!("error5");
        }


        // let mut cost2 = unit;
        // for sub_node in &unique_elements{
        //     cost2 += costs(Id::from(*sub_node)).1;
        //     println!("costs {:?} for {:?} -- {:?}", sub_node, costs(Id::from(*sub_node)).1,cost2);
        // }

        // println!("    output:{:?} with cost {} for enode {:?}",unique_elements, node_size, enode);

        // if node_size!=cost2 {
        //     println!("wrong cost: {:?}, {:?}", node_size, cost2);
        //     println!("unique_elements: {:?}", unique_elements);
        //     for node in unique_elements.clone(){
        //         println!("{:?}---{:?}",node,costs(Id::from(node)));
        //     }
        //     panic!("error1");
        // }


        // println!("node_size: {}", node_size);
        // let duration1 = start1.elapsed();
        // let time1 = duration1.as_secs() as f64 + duration1.subsec_nanos() as f64 * 1e-9;
        // self.time1=self.time1+time1;
        println!("{:?},{:?}",self.time2,self.time1);
        (node_size,unit,combined)
    }
}


// pub struct Modified_AstSizeCost;

// impl CostFunction<SimpleLanguage> for Modified_AstSizeCost {
//     type Cost = (usize,Vec<Id>);

//     fn cost<C>(&mut self, enode: &SimpleLanguage, mut costs: C) -> Self::Cost
//     where
//         C: FnMut(Id) -> Self::Cost,
//     {
//         let mut sum = 1;
        
//         for id in enode.children() {
//             sum += costs(*id).0;
//         }

//         let mut combined:Vec<Id> = Vec::new();
//         for id in enode.children() {
//             combined.extend(costs(*id).1.clone());
//         }
//         for id in enode.children() {
//             combined.push(*id);
//         }
//         println!("{:?}|{:?}|{:?}",enode,enode.children(),combined);

//         let mut counts = HashMap::new();
//         for value in &combined {
//             *counts.entry(*value).or_insert(0) += 1;
//         }

//         let set: HashSet<_> = combined.into_iter().collect();
//         let mut unique_elements: Vec<Id> = set.into_iter().collect();

//         let duplicates: HashMap<_, _> = counts.iter()
//         .filter(|&(_, &count)| count > 1)
//         .map(|(&k, &v)| (k, v))
//         .collect();


//         for (&element, &count) in duplicates.iter() {
//             sum -= (count - 1) * 1;
//             println!("Element {} has {} occurrences.", element, count);
//         }

//         (sum,unique_elements)

//     }
// }


fn simplify_programmatically(expr: &RecExpr<SimpleLanguage>) -> (std::time::Duration,std::time::Duration,std::time::Duration,usize, RecExpr<SimpleLanguage>, RecExpr<SimpleLanguage>,Runner<SimpleLanguage,()>){
    println!("Start E-graph Grownth");
    let start = Instant::now();
    let runner = Runner::default().with_time_limit(Duration::from_secs(1000000000)).with_iter_limit(1000000000).with_node_limit(70000).with_expr(expr).run(&static_rules::make_rules());
    println!(" Stop Reason: {:?}",runner.stop_reason);
    let grownth_duration = start.elapsed();
    println!("      Saturation: {:?}",grownth_duration);
    let start = Instant::now();
    println!("Start Greedy  Extraction");
    let extractor = Extractor::new(&runner.egraph, WeightedAstSize{time1: 0.0, time2: 0.0 });
    let root = runner.roots[0];
    let (best_cost, best) = extractor.find_best(root);
    let greedy_duration = start.elapsed();
    let cells = lib::parse_genlib("7nm.genlib");


    println!("best_cost:{:?}",best_cost);
    println!("{:?}",WeightedAstSize{time1: 0.0, time2: 0.0}.cost_rec(&best));
    // let start = Instant::now();
    // println!("Start LP  Extraction");
    // let lp_best = lp::LpExtractor::new(&runner.egraph, AstSize,cells).solve(root);
    // let lp_duration = start.elapsed();

    (grownth_duration, greedy_duration, greedy_duration, best_cost.0, best.clone(), best,runner)
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <benchmark>", args[0]);
        std::process::exit(1);
    }
    let benchmark = &args[1];
    // let benchmark = "c2670";
    let path = "benchmark/iscas85/";
    let blif_path = &format!("benchmark/output/{}/",benchmark);

    // check and create the folder
    let blif_dir = Path::new(&blif_path);
    if !blif_dir.exists() {
        if let Err(e) = fs::create_dir_all(blif_dir) {
            eprintln!("Error creating directory {}: {}", blif_path, e);
            std::process::exit(1);
        }
    }

    // match lib::parse_graph("benchmark/iscas85/c7552.bench") {
    let input_file_path=format!("{}{}.bench", path, benchmark);
    match lib::parse_graph(&input_file_path) {
        Ok((vertices, _, in_pairs, _ ,edge_pairs, input, output)) => {
            let mut expr: RecExpr<SimpleLanguage> = RecExpr::default();
            let mut vertices_map: HashMap<String, Id> = HashMap::new();
            let mut queue: Vec<String> = vertices.clone();
            let mut out_id: HashMap<String, Id> = HashMap::new();
            queue.reverse();
            while !queue.is_empty(){
                let operation=queue.pop().unwrap();
                if let Some(input_edges) = in_pairs.get(&operation) {
                    let mut in_id = Vec::new();
                    for vertex in edge_pairs.get(input_edges).unwrap_or(&Vec::new()){
                        if let Some(id) = vertices_map.get(vertex) {
                            in_id.push(*id);
                        }
                    }
                    let name: Vec<&str> = input_edges.split("_").collect();
                    if name.len()==2 {
                        let stripped_name = name.first().unwrap().to_lowercase();
                        if let Some(value) = convert_to_simple_language_enum (in_id, &stripped_name) {
                            let temp_id = expr.add(value);
                            if output.contains(&operation){
                                out_id.insert(operation.clone(), temp_id);
                            }
                            vertices_map.insert(operation.clone(), temp_id);
                            queue.retain(|x| x != &operation);
                        }
                        else {
                            panic!("Unknown enum variant for {}", stripped_name);
                        }
                    }
                    else {
                        panic!("Edge format error: {}", input_edges);
                    }
                } else {
                    let temp_id = expr.add(SimpleLanguage::Symbol(operation.clone().into()));
                    if output.contains(&operation){
                        out_id.insert(operation.clone(), temp_id);
                    }
                    vertices_map.insert(operation.clone(), temp_id);
                    queue.retain(|x| x != &operation);
                }
            }

            let mut vec_out_id: Vec<Id> = Vec::new();
            for element in &output{
                vec_out_id.push(out_id[element]);
            }

            let value = SimpleLanguage::OUT(vec_out_id);
            let temp_id = expr.add(value);
            vertices_map.insert("output".to_string(), temp_id);

            
            let (grownth_duration, greedy_duration, lp_duration, cost, greedy_best_expr, lp_best_expr, runner) = simplify_programmatically(&expr);

            // let expr_check_1: RecExpr<SimpleLanguage> ="(or (not (or (not A) (not C))) (not (or (not B) (and C E))))".parse().unwrap();
            // let (_, _, _, _, _, _, runner1) = simplify_programmatically(&expr_check_1);
            // let expr_check_2: RecExpr<SimpleLanguage> ="(or (and A C) (or (and B (not C)) (and B (not E))))".parse().unwrap();
            // println!("123456:{:?}",runner1.egraph.equivs(&expr_check_1,&expr_check_2));

            // let mut file = File::create("greedy_output.txt").expect("Failed to create file");
            // write!(file, "{}", greedy_best_expr).expect("Failed to write data to file");
            // let mut file = File::create("LP_output.txt").expect("Failed to create file");
            // write!(file, "{}", lp_best_expr).expect("Failed to write data to file");
            let mut greedy_counts: HashMap<&str, usize> = HashMap::new();
            for node in greedy_best_expr.as_ref() {
                let node_type = get_node_type(node);
                *greedy_counts.entry(node_type).or_insert(0) += 1;
            }
            // println!("{:?}",greedy_counts);
            let mut lp_counts: HashMap<&str, usize> = HashMap::new();
            for node in lp_best_expr.as_ref() {
                let node_type = get_node_type(node);
                *lp_counts.entry(node_type).or_insert(0) += 1;
            }

            println!("----------------------------------------------------------------------");
            println!("Runtime:");
            println!("      Saturation: {:?}",grownth_duration);
            println!("      Greedy    : {:?}",greedy_duration);
            println!("      ILP       : {:?}",lp_duration);

            // println!("Expr:");
            // println!("      Greedy: {:?}",greedy_best_expr);
            // println!("      ILP   : {:?}",lp_best_expr);
            // println!(" Original   : {:?}",expr);

            // if let Err(e) = lib::parse2blif(&blif_path, &benchmark, &expr,input,output) {
            //     eprintln!("Failed to parse to BLIF: {}", e);
            // }

            
            if let Err(e) = lib::parse2blif(&blif_path, &benchmark, &greedy_best_expr,input,output) {
                eprintln!("Failed to parse to BLIF: {}", e);
            }

            // println!("Counts:");
            // println!("      Greedy: {:?}",greedy_counts);
            // println!("      Count : {:?}",lp_counts);
            println!("----------------------------------------------------------------------");




            let command = format!(
                "abc -c \"read 7nm.genlib;read {0};map;map;map;map;map;ps;write {1};read {2};ps;cec {2} {1}\"",
                input_file_path, format!("{}{}_map.blif", blif_path, benchmark), format!("{}{}.blif", blif_path, benchmark)
            );
        
            let output = Command::new("sh")
                .arg("-c")
                .arg(&command)
                .output()
                .expect("Failed to execute command");
        
            // 检查命令执行的结果
            if !output.status.success() || !output.stderr.is_empty() {
                eprintln!(
                    "Command execution failed with status: {}",
                    output.status
                );
                if !output.stderr.is_empty() {
                    eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
                }
                std::process::exit(1);
            }
            println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            if !output.stderr.is_empty() {
                println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
            }
        },
        Err(e) => {
            // Handle the error here
            println!("Failed to parse the graph: {}", e);
        }
    }
}
