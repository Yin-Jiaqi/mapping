use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use std::fs::File;
use egg::RecExpr;
use egg::Language;
use regex::Regex;
use std::io::{self, BufRead, BufReader, Write, Error,BufWriter};
use egraph_mapping::{SimpleLanguage,get_input_list};

use egg::*;
use extraction_gym::*;
use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_str,Value};


fn topological_sort_or_cycle(
    vertices: Vec<String>, 
    in_pairs: HashMap<String, String>, 
    out_pairs: HashMap<String, Vec<String>>
) -> Result<Vec<String>, Vec<String>> {
    let mut graph = HashMap::new();
    let mut in_degree = HashMap::new();
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    // 初始化所有顶点的入度为0
    for vertex in &vertices {
        in_degree.insert(vertex.clone(), 0);
    }

    // 构建图和计算入度
    for vertex in &vertices {
        // 找到由当前顶点产生的所有边，然后找到这些边连接的顶点
        if let Some(edges) = out_pairs.get(vertex) {
            for e in edges {
                // 查找由边e生成的顶点
                let target_vertices = in_pairs.iter().filter_map(|(v, ed)| if ed == e { Some(v.clone()) } else { None }).collect::<Vec<_>>();
                for target_vertex in target_vertices {
                    graph.entry(vertex.clone()).or_insert_with(Vec::new).push(target_vertex.clone());
                    *in_degree.entry(target_vertex.clone()).or_insert(0) += 1;
                }
            }
        }
    }


    // 将所有入度为0的节点入队列
    for (vertex, deg) in &in_degree {
        if *deg == 0 {
            queue.push_back(vertex.clone());
        }
    }


    // 执行Kahn算法
    while let Some(vertex) = queue.pop_front() {
        order.push(vertex.clone());
        if let Some(neighbors) = graph.get(&vertex) {
            for neighbor in neighbors {
                let degree = in_degree.get_mut(neighbor).unwrap();
                *degree -= 1;
                if *degree == 0 {
                    queue.push_back(neighbor.clone());
                }
            }
        }
    }

    // 检查是否所有的顶点都在排序中，如果不是则存在环
    if order.len() != vertices.len() {
        // 存在环
        return Err(find_cycle(vertices.clone(), in_pairs, out_pairs));
    }

    Ok(order)
}


fn find_cycle(
    vertices: Vec<String>, 
    in_pairs: HashMap<String, String>, 
    out_pairs: HashMap<String, Vec<String>>
) -> Vec<String> {
    let mut visited = HashSet::new();
    let mut stack = HashSet::new();
    let mut parent = HashMap::new();
    let mut graph = HashMap::new();

    // 构建图的映射
    for vertex in &vertices {
        if let Some(edges) = out_pairs.get(vertex) {
            for edge in edges {
                if let Some(next_vertex) = in_pairs.iter().find(|&(_, e)| e == edge).map(|(v, _)| v) {
                    graph.entry(vertex.clone()).or_insert_with(Vec::new).push(next_vertex.clone());
                }
            }
        }
    }

    for vertex in &vertices {
        if !visited.contains(vertex) {
            if let Some(cycle) = dfs(vertex, &mut visited, &mut stack, &mut parent, &graph) {
                return cycle;
            }
        }
    }

    vec![]  // 如果没有找到环，返回空向量
}

fn dfs(
    vertex: &String, 
    visited: &mut HashSet<String>, 
    stack: &mut HashSet<String>,
    parent: &mut HashMap<String, String>,
    graph: &HashMap<String, Vec<String>>
) -> Option<Vec<String>> {
    visited.insert(vertex.clone());
    stack.insert(vertex.clone());

    if let Some(neighbors) = graph.get(vertex) {
        for neighbor in neighbors {
            if !visited.contains(neighbor) {
                parent.insert(neighbor.clone(), vertex.clone());
                if let Some(cycle) = dfs(neighbor, visited, stack, parent, graph) {
                    return Some(cycle);
                }
            } else if stack.contains(neighbor) {
                // 发现环
                return Some(construct_cycle(neighbor, vertex, parent));
            }
        }
    }

    stack.remove(vertex);
    None
}

fn construct_cycle(
    start: &String,
    current: &String,
    parent: &HashMap<String, String>
) -> Vec<String> {
    let mut cycle = Vec::new();
    let mut node = current;
    cycle.push(node.clone());
    while node != start {
        if let Some(p) = parent.get(node) {
            node = p;
            cycle.push(node.clone());
        }
    }
    cycle.push(start.clone()); // 完成环
    cycle.reverse();  // 反转使其从开始节点开始
    cycle
}


pub fn parse_graph(input: &str) -> io::Result<(Vec<String>, Vec<String>, HashMap<String, String>, HashMap<String, Vec<String>>, HashMap<String, Vec<String>>, Vec<String>, Vec<String>)> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);

    let mut output_data = Vec::new();
    let mut input_data = Vec::new();
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut operation_index = HashMap::<String, usize>::new();
    let mut out_pairs = HashMap::<String, Vec<String>>::new();
    let mut in_pairs = HashMap::<String, String>::new();
    let mut edge_pairs = HashMap::<String, Vec<String>>::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("OUTPUT("){
            let parts: Vec<_> = line.split('(').collect();
            let var_name = parts[1].trim_end_matches(')');
            if !vertices.contains(&var_name.to_string()) {
                vertices.push(var_name.to_string());
            }
            if !output_data.contains(&var_name.to_string()) {
                output_data.push(var_name.to_string());
            }
            continue;
        }
        if line.starts_with("INPUT(") {
            let parts: Vec<_> = line.split('(').collect();
            let var_name = parts[1].trim_end_matches(')');
            if !vertices.contains(&var_name.to_string()) {
                vertices.push(var_name.to_string());
            }
            if !input_data.contains(&var_name.to_string()) {
                input_data.push(var_name.to_string());
            }
            continue;
        }

        let parts: Vec<_> = line.split(" = ").collect();
        if parts.len() == 2 {
            let left = parts[0].trim();
            let right = parts[1].trim();

            if !vertices.contains(&left.to_string()) {
                vertices.push(left.to_string());
            }
            

            let op_parts: Vec<_> = right.split('(').collect();
            if op_parts.len() == 2 {
                let operation = op_parts[0].trim();
                let operands = op_parts[1].trim().trim_end_matches(')').split(',').map(|s| s.trim().to_string()).collect::<Vec<String>>();
                
                let index = operation_index.entry(operation.to_string()).or_insert(0);
                let edge_name = format!("{}_{}", operation.to_lowercase(), *index);
                edge_pairs.insert(edge_name.clone(),operands.clone());
                edges.push(edge_name.clone());
                *index += 1;

                for operand in operands.iter() {
                    if !vertices.contains(&operand.to_string()) {
                        vertices.push(operand.to_string());
                    }
                    out_pairs.entry(operand.clone()).or_insert_with(Vec::new).push(edge_name.clone());
                }
                in_pairs.insert(left.to_string(), edge_name);
            }
        }
    }

    match topological_sort_or_cycle(vertices.clone(), in_pairs.clone(), out_pairs.clone()) {
        Ok(sorted) => {
            Ok((sorted, edges, in_pairs, out_pairs,edge_pairs,input_data,output_data))
        }
        Err(cycle) => {
            println!("Detected cycle: {:?}", cycle);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Cycle detected"))
        }
    }
}


pub fn parse_blif(input: &str) -> io::Result<(Vec<String>, Vec<String>, HashMap<String, String>, HashMap<String, Vec<String>>, HashMap<String, Vec<String>>, Vec<String>, Vec<String>)> {
    let file = File::open(input)?;
    let mut reader = BufReader::new(file);

    let mut output_data = Vec::new();
    let mut input_data = Vec::new();
    let mut vertices = Vec::new();
    let mut edges = Vec::new();
    let mut operation_index = HashMap::<String, usize>::new();
    let mut out_pairs = HashMap::<String, Vec<String>>::new();
    let mut in_pairs = HashMap::<String, String>::new();
    let mut edge_pairs = HashMap::<String, Vec<String>>::new();

    let mut current_line = String::new();

    while reader.read_line(&mut current_line)? > 0 {
        let mut line = current_line.trim().to_string();
        while line.ends_with('\\') {
            line.pop(); // 去掉行末的反斜杠
            current_line.clear();
            reader.read_line(&mut current_line)?;
            line.push_str(current_line.trim());
        }
        // println!("{:?}",line);

        if line.starts_with(".inputs"){
            let parts: Vec<_> = line.split_whitespace().skip(1).collect();
            input_data.extend(parts.iter().map(|s| s.to_string()));
            vertices.extend(parts.iter().map(|s| s.to_string()));
        }


        if line.starts_with(".outputs"){
            let parts: Vec<_> = line.split_whitespace().skip(1).collect();
            output_data.extend(parts.iter().map(|s| s.to_string()));
            vertices.extend(parts.iter().map(|s| s.to_string()));
        }

        if line.starts_with(".gate"){
            let parts: Vec<_> = line.split_whitespace().skip(1).collect();
            let operation=parts[0];
            let result = parts.last().map_or_else(
                || {
                    println!("parts: {:?}", parts);
                    panic!("The parts vector is empty")
                },
                |last_part| {
                    last_part.split('=').last().map_or_else(
                        || {
                            println!("parts: {:?}", parts);
                            panic!("The last element does not contain an '='")
                        },
                        |value| value
                    )
                }
            );

            let oprands: Vec<String> = parts
            .iter()
            .skip(1)                      // 跳过第一个元素
            .take(parts.len().saturating_sub(2))  // 只处理直到倒数第二个元素
            .filter_map(|part| part.split('=').last())  // 对每个元素进行分割，并取最后一部分
            .map(|s| s.to_string())        // 将 &str 转换为 String
            .collect();                    // 收集结果到一个新的 Vec 中

            if !vertices.contains(&result.to_string()) {
                vertices.push(result.to_string());
            }

            let index = operation_index.entry(operation.to_string()).or_insert(0);
            let edge_name = format!("{}_{}", operation, *index);
            edge_pairs.insert(edge_name.clone(),oprands.clone());
            if !edges.contains(&edge_name) {
                edges.push(edge_name.clone());
            }
            *index += 1;

            for operand in oprands.iter() {
                if !vertices.contains(&operand.to_string()) {
                    vertices.push(operand.to_string());
                }
                out_pairs.entry(operand.clone()).or_insert_with(Vec::new).push(edge_name.clone());
            }
            in_pairs.insert(result.to_string(), edge_name);

        }
        // if line.starts_with(".inputs"){
        //     current_line.push_str(&line);
        //     let parts: Vec<_> = current_line.split_whitespace().skip(1).collect();
        //     output_data.extend(parts.iter().map(|s| s.to_string()));
        //     vertices.extend(parts.iter().map(|s| s.to_string()));
        //     continue;
        // }
        current_line.clear();
    }

    // println!("in_pairs:{:?}",in_pairs);
    // println!("edge_pairs:{:?}",edge_pairs);
    match topological_sort_or_cycle(vertices.into_iter().collect(), in_pairs.clone(), out_pairs.clone()) {
        Ok(sorted) => {
            Ok((sorted, edges.into_iter().collect(), in_pairs, out_pairs,edge_pairs,input_data.into_iter().collect(),output_data.into_iter().collect()))
        }
        Err(cycle) => {
            println!("Detected cycle: {:?}", cycle);
            Err(std::io::Error::new(std::io::ErrorKind::Other, "Cycle detected"))
        }
    }
}

pub fn parse_genlib(file_path: &str) -> Vec<(String, usize, usize, String)> {
    let mut cells = Vec::new();
    let mut current_cell: (_, usize, usize, _) = (String::new(), 100, 100, String::new());

    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open file: {}", e);
            return cells; // Return empty vector if the file cannot be opened.
        }
    };

    let reader = io::BufReader::new(file);
    let gate_re = Regex::new(r"^GATE (\S+)\s+(\d+\.\d+)?\s+(.*);").unwrap();
    let delay_re = Regex::new(r"^DELAY \S+ \S+ (\d+\.\d+)").unwrap();

    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                continue; // Skip lines that can't be read.
            }
        };

        if let Some(caps) = gate_re.captures(&line) {
            if !current_cell.0.is_empty() {
                cells.push(current_cell.clone());
            }
            current_cell = (
                caps[1].to_string(),
                (caps.get(2).map_or(1.0, |m| m.as_str().parse().unwrap_or(1.0)) as f64 * 100.0) as usize,
                100,
                caps[3].to_string(),
            );
        }

        if let Some(caps) = delay_re.captures(&line) {
            current_cell.2 = (caps[1].parse().unwrap_or(1) as f64 * 100.0) as usize;
        }
    }

    if !current_cell.0.is_empty() {
        cells.push(current_cell);
    }

    cells
}



// fn find_duplicates(output_vec: Vec<usize>, output_index: &HashMap<usize, String>) -> HashMap<usize, Vec<&String>> {
//     let mut index_map: HashMap<usize, Vec<&String>> = HashMap::new();

//     for (index, &value) in output_vec.iter().enumerate() {
//         if let Some(output_str) = output_index.get(&index) {
//             index_map.entry(value).or_insert(Vec::new()).push(output_str);
//         }
//     }

//     index_map.retain(|_, v| v.len() > 1);
//     index_map
// }

fn find_duplicates(output_vec: Vec<usize>, output_index: &Vec<String>) -> HashMap<usize, Vec<&String>> {
    let mut index_map: HashMap<usize, Vec<&String>> = HashMap::new();

    for (index, &value) in output_vec.iter().enumerate() {
        if let Some(output_str) = output_index.get(index) {
            index_map.entry(value).or_insert(Vec::new()).push(output_str);
        }
    }

    index_map.retain(|_, v| v.len() > 1);
    index_map
}



pub fn parse2blif(file_path: &str, benchmark: &str, expr: &RecExpr<SimpleLanguage>, input: Vec<String>, output: Vec<String>) -> Result<(), Error> {
    let model_name = &format!(".model {}_EGG\n", benchmark);
    let input = format!(".inputs {}\n", input.join(" "));
    let output_str = format!(".outputs {}\n", output.join(" "));

    // let file_path=&format!("{}{}.blif", path, benchmark);

    let mut file = File::create(file_path)?;
    file.write_all(model_name.as_bytes())?;
    file.write_all(input.as_bytes())?;
    file.write_all(output_str.as_bytes())?;

    let mut node_index = HashMap::new();
    let mut temp_index=1;
    let mut output_vec = Vec::new();

    for node in expr.get_nodes(){
        match node{
            SimpleLanguage::OUT(_) => {
                for (_,input) in node.children().iter().enumerate(){
                    output_vec.push(usize::from(*input));
                }
            }
            _ => {}
        }
    }

    for (index, node) in expr.get_nodes().iter().enumerate(){
        match node{
            SimpleLanguage::Symbol(_) => {
                node_index.insert(index,format!("{}", node));
                if let Some(position) = output_vec.iter().position(|&x| x == index) {
                    if format!("{}",node) != format!("{}",output[position]){
                        let gate_line = format!(".gate HB1xp67_ASAP7_75t_L A={} Y={}\n", node, output[position]);
                        file.write_all(gate_line.as_bytes())?;
                    }
                }
            },
            SimpleLanguage::OUT(_) => {
            },
            _ => {
                let cell_input = get_input_list(format!("{}", node));
                if node.children().len()!=cell_input.len()-1{
                    panic!("Input length not match, {},{}",node.children().len(),cell_input.len());
                }
                // println!("{:?}",node.children());
                // println!("{:?}",(*node).children_mut());
                let mut gate_line = format!(".gate {} ", node);
                for (indice,input) in node.children().iter().enumerate(){
                    gate_line.push_str(cell_input[indice]);
                    gate_line.push_str("=");
                    // println!("{:?}",Id::from(*input));
                    // let _type_checker: usize = usize::from(*input);
                    let input_node = match node_index.get(&usize::from(*input)){
                        Some(node) => node,
                        None => panic!("Node not found for index {}", index),
                    };
                    gate_line.push_str(&input_node);
                    gate_line.push_str(" ");
                }
                gate_line.push_str(cell_input.last().unwrap());
                gate_line.push_str("=");
                // println!("4444-{:?}",expr.get_nodes()[1080]);
                // println!("3333-{:?}",output_vec);
                // println!("2222-{:?}",index);
                // if output_vec.contains(&index) {
                //     println!("1111-{:?}-{}-{}",output_size,output_vec.len(),index);
                //     output_size=output_size+1;
                // }
                
                match output_vec.iter().position(|&x| x == index) {
                    Some(pos) => {
                        gate_line.push_str(&format!("{}\n", output[pos]));
                        node_index.insert(index,format!("{}", output[pos]));
                    }
                    None => {                    
                        gate_line.push_str(&format!("temp_{}\n", temp_index));
                        node_index.insert(index,format!("temp_{}", temp_index));
                        temp_index=temp_index+1;
                    }
                }
                // if output_vec.contains(&index) {
                //     gate_line.push_str(&format!("{}\n", output[output_index]));
                //     node_index.insert(index,format!("{}", output[output_index]));
                //     output_index=output_index+1;
                // }
                // else{
                //     gate_line.push_str(&format!("temp_{}\n", temp_index));
                //     node_index.insert(index,format!("temp_{}", temp_index));
                //     temp_index=temp_index+1;
                // }

                file.write_all(gate_line.as_bytes())?;
            }
        }
    }

    let duplicates = find_duplicates(output_vec.clone(),&output);


    for (_, indices) in &duplicates {
        for i in 1..=indices.len()-1 {
            let gate_line = format!(".gate HB1xp67_ASAP7_75t_L A={} Y={}\n", indices[0], indices[i]);
            file.write_all(gate_line.as_bytes())?;
        }
    }



    file.write_all(".end\n".as_bytes())?;


    Ok(())
}


pub fn merge_sorted_vecs_unique(v1: Vec<usize>, v2: Vec<usize>) -> (Vec<usize>, HashMap<usize, usize>) {
    let mut result = Vec::with_capacity(v1.len() + v2.len());  // 预分配足够的空间
    let mut temp_counts = HashMap::new();  // 临时存储元素的出现次数
    let mut counts = HashMap::new();  // 最终只存储重复元素的计数
    let mut i = 0;  // v1的索引
    let mut j = 0;  // v2的索引

    // 当两个向量都还有元素时
    while i < v1.len() && j < v2.len() {
        let current = if v1[i] < v2[j] {
            let temp = v1[i];
            i += 1;
            temp
        } else {
            let temp = v2[j];
            j += 1;
            temp
        };

        let count = temp_counts.entry(current).or_insert(0);
        *count += 1;
        if *count == 1 {
            result.push(current);
        } else if *count == 2 {
            counts.insert(current, 2);
        } else {
            *counts.get_mut(&current).unwrap() += 1;
        }
    }

    // 处理任一向量中的剩余元素
    fn process_remaining_elements(vec: &[usize], i: &mut usize, temp_counts: &mut HashMap<usize, usize>, counts: &mut HashMap<usize, usize>, result: &mut Vec<usize>) {
        while *i < vec.len() {
            let current = vec[*i];
            let count = temp_counts.entry(current).or_insert(0);
            *count += 1;
            if *count == 1 {
                result.push(current);
            } else if *count == 2 {
                counts.insert(current, 2);
            } else {
                *counts.get_mut(&current).unwrap() += 1;
            }
            *i += 1;
        }
    }

    process_remaining_elements(&v1, &mut i, &mut temp_counts, &mut counts, &mut result);
    process_remaining_elements(&v2, &mut j, &mut temp_counts, &mut counts, &mut result);

    (result, counts)
}



pub fn merge_and_update_maps(map1: &mut HashMap<usize, usize>, map2: &HashMap<usize, usize>) {
    // 遍历第二个哈希图
    for (key, _) in map2 {
        // 检查这个键是否也在第一个哈希图中
        match map1.get_mut(key) {
            Some(value1) => {
                // 如果在两个哈希图中都有，将它们的值相加并更新第一个哈希图
                *value1 += 1;
            },
            None => {
                // 如果在第一个哈希图中没有找到该键，将其从第二个哈希图中插入到第一个哈希图中
                map1.insert(*key, 2);
            }
        }
    }
}

pub fn insert_sorted(vec: &mut Vec<usize>, value: usize, duplicates: &mut HashMap<usize, usize>) {
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
