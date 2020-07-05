use crate::ConnectionGene;
use crate::NodeGene;
use crate::NodeGeneType;
use crate::Genome;

use image::{Rgb, RgbImage};
use imageproc::drawing::*;
use imageproc::rect::*;
use rusttype::{Font, Scale};

use std::path::Path;

pub struct GenomePrinter {}

impl GenomePrinter {
    pub fn new() -> Self {
        Self {}
    }

    pub fn print_genome(&mut self, genome: &mut Genome, path: &str, name: &str) {
        let timer = std::time::Instant::now();

        let path = format!("./output/{}.png", path);
        let path = Path::new(&path);
        let width = 800;
        let height = 800;

        let mut image = RgbImage::new(width, height);

        let red = Rgb([255u8, 187u8, 177u8]);
        let blue = Rgb([193u8, 228u8, 247u8]);
        let white = Rgb([255u8, 255u8, 255u8]);
        let black = Rgb([0u8, 0u8, 0u8]);
        let yellow = Rgb([255u8, 236u8, 169u8]);
        let expressed_yellow = Rgb([255u8, 179u8, 0u8]);

        // Clear the screen
        draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(width, height), white);
        // Get the font - Compiled into the build
        let font = Font::from_bytes(include_bytes!("../SourceSansPro-Bold.ttf") as &[u8]).unwrap();

        // Font settings
        let font_height = 26.0;
        let font_scale = Scale {
            x: font_height * 1.0,
            y: font_height,
        };

        // Name font settings
        let name_font_height = 60.0;
        let name_font_scale = Scale {
            x: name_font_height * 1.0,
            y: name_font_height,
        };

        // Draw the name in the center
        draw_text_mut(&mut image, black, 80, 80, name_font_scale, &font, name);

        // Array used for keeping track of node locations on the grid
        let mut node_locations: Vec<PrintNodeLocation> = Vec::new();

        // Draw inputs
        {
            let nodes = genome.get_node_genes();
            let mut nodes: Vec<NodeGene> = nodes.iter().map(|(_, node)| node.clone()).collect();

            nodes.sort_by(|a, b| b.get_id().cmp(&a.get_id()));

            let inputs = nodes.iter().filter(|x| x.get_type() == NodeGeneType::INPUT);

            let input_count = inputs.to_owned().count() as i32;
            let mut current = 0;

            for node in inputs {
                let mut pos = 600.0 * (current as f32 / input_count as f32);
                if pos == 0.0 {
                    pos = 1.0;
                }
                pos = 600.0 - pos;

                // Draw circle
                draw_filled_circle_mut(&mut image, (pos as i32, 600), 18, black);
                draw_filled_circle_mut(&mut image, (pos as i32, 600), 16, yellow);
                node_locations.push(PrintNodeLocation::new(
                    node.get_id(),
                    pos as i32,
                    600,
                    NodeGeneType::INPUT,
                ));

                // Draw numbers
                draw_text_mut(
                    &mut image,
                    black,
                    pos as u32 - 5,
                    600 - 10,
                    font_scale,
                    &font,
                    &(node.get_id()).to_string(),
                );

                // Add to current
                current += 1;
            }
        }

        // Draw hidden
        {
            let nodes = genome.get_node_genes();
            let mut nodes: Vec<NodeGene> = nodes.iter().map(|(_, node)| node.clone()).collect();

            nodes.sort_by(|a, b| b.get_id().cmp(&a.get_id()));

            let hidden = nodes
                .iter()
                .filter(|x| x.get_type() == NodeGeneType::HIDDEN);

            let hidden_count = hidden.to_owned().count() as i32;
            let mut current = 0;

            for node in hidden {
                let mut pos = 600.0 * (current as f32 / hidden_count as f32);
                if pos == 0.0 {
                    pos = 1.0;
                }
                pos = 600.0 - pos;

                let mut y_pos = 400;

                let mut last_was_moved = false;
                for node_location in &node_locations {
                    if node_location.node_type == NodeGeneType::HIDDEN
                        && node_location.y == y_pos as i32
                    {
                        if !last_was_moved {
                            y_pos -= 70;

                            last_was_moved = true;
                        } else {
                            y_pos = 400;

                            last_was_moved = false;
                        }
                    }
                }

                // Draw circle
                draw_filled_circle_mut(&mut image, (pos as i32, y_pos), 18, black);
                draw_filled_circle_mut(&mut image, (pos as i32, y_pos), 16, blue);
                node_locations.push(PrintNodeLocation::new(
                    node.get_id(),
                    pos as i32,
                    y_pos,
                    NodeGeneType::HIDDEN,
                ));

                // Draw numbers
                draw_text_mut(
                    &mut image,
                    black,
                    pos as u32 - 5,
                    (y_pos - 10) as u32,
                    font_scale,
                    &font,
                    &(node.get_id()).to_string(),
                );

                // Add to current
                current += 1;
            }
        }

        // Draw outputs
        {
            let nodes = genome.get_node_genes();
            let mut nodes: Vec<NodeGene> = nodes.iter().map(|(_, node)| node.clone()).collect();

            nodes.sort_by(|a, b| b.get_id().cmp(&a.get_id()));

            let outputs = nodes
                .iter()
                .filter(|x| x.get_type() == NodeGeneType::OUTPUT);

            let outputs_count = outputs.to_owned().count() as i32;
            let mut current = 0;

            for node in outputs {
                let mut pos = 600.0 * (current as f32 / outputs_count as f32);
                if pos == 0.0 {
                    pos = 1.0;
                }
                pos = 600.0 - pos;

                for node_location in &node_locations {
                    if node_location.node_type == NodeGeneType::HIDDEN
                        && node_location.x == pos as i32
                    {
                        pos -= 70.0;
                    }
                }

                // Draw circle
                draw_filled_circle_mut(&mut image, (pos as i32, 200), 18, black);
                draw_filled_circle_mut(&mut image, (pos as i32, 200), 16, red);
                node_locations.push(PrintNodeLocation::new(
                    node.get_id(),
                    pos as i32,
                    200,
                    NodeGeneType::OUTPUT,
                ));

                // Draw numbers
                draw_text_mut(
                    &mut image,
                    black,
                    pos as u32 - 5,
                    200 - 10,
                    font_scale,
                    &font,
                    &(node.get_id()).to_string(),
                );

                // Add to current
                current += 1;
            }
        }

        // Add lines
        let mut current = 0;
        let connection_count = genome.get_connection_genes().values().count();
        for con in genome.get_connection_genes().values() {
            let con: &ConnectionGene = con;

            let in_node = con.get_in_node();
            let out_node = con.get_out_node();

            let mut in_location = &PrintNodeLocation::new(0, 0, 0, NodeGeneType::HIDDEN);
            let mut out_location = in_location;

            for node_location in &node_locations {
                if in_node == node_location.id {
                    in_location = node_location;
                } else if out_node == node_location.id {
                    out_location = node_location;
                }
            }

            // Draw black if it's enabled else yellow
            if con.is_expressed() {
                draw_line_segment_mut(
                    &mut image,
                    ((in_location.x) as f32, (in_location.y) as f32),
                    ((out_location.x) as f32, (out_location.y) as f32),
                    black,
                );
            } else {
                draw_line_segment_mut(
                    &mut image,
                    ((in_location.x) as f32, (in_location.y) as f32),
                    ((out_location.x) as f32, (out_location.y) as f32),
                    expressed_yellow,
                );
            }

            // Weights
            let mut pos = 600.0 * (current as f32 / connection_count as f32);
            if pos == 0.0 {
                pos = 1.0;
            }
            pos = 600.0 - pos;

            draw_text_mut(
                &mut image,
                black,
                pos as u32 - 5,
                60 - 10,
                font_scale,
                &font,
                &(con.get_weight()).to_string(),
            );

            current += 1;
        }

        image.save(path).unwrap();

        let time_taken = timer.elapsed();
        println!("Save time for {} took {:?}", name, time_taken);
    }
}

pub struct PrintNodeLocation {
    id: i32,

    x: i32,
    y: i32,

    node_type: NodeGeneType,
}

impl PrintNodeLocation {
    pub fn new(id: i32, x: i32, y: i32, node_type: NodeGeneType) -> PrintNodeLocation {
        PrintNodeLocation {
            id,
            x,
            y,
            node_type,
        }
    }
}
