#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::egui;
use egui_extras::{TableBuilder, Column};
use serde::{Serialize, Deserialize};

use std::fs::{read_to_string, write};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("Egui CRUD", native_options, Box::new(|cc| Box::new(Application::new(cc))))
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
#[serde(default)]
struct Product {
    id: usize,
    product_name: String,
    value: f32,
}

impl Product {
    fn new(id: usize, name: String, value: f32) -> Self {
        Self {
            id: id,
            product_name: name,
            value: value,
        }
    }
}

struct Application {
    // a product to hold the values for our new products
    cur_product: Product,
    // used as the ID for our new products
    cur_index: usize,
    products: Vec<Product>,
    old_products: Vec<Product>,
    search: String,
    searching: bool,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            cur_product: Product::new(0, "Product".to_string(), 0.0),
            cur_index: 0,
            products: Vec::new(),
            old_products: Vec::new(),
            search: String::new(),
            searching: false,
        }
    }
}

impl Application {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for Application {
   fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down_justified(egui::Align::Center), |ui| {
                if self.products.last().is_none() {
                    self.cur_index = 0;
                }
                else {
                    self.cur_index = self.products.last().unwrap().id + 1;
                }
                let heading_text = egui::RichText::new("Products").font(egui::FontId::proportional(40.0));
                ui.heading(heading_text);
                ui.separator();
                ui.add_space(10.0);
                // product id
                ui.label("Product ID: ");
                ui.label(self.cur_index.to_string());
                // product name
                let name_label = ui.label("Product Name: ");
                ui.text_edit_singleline(&mut self.cur_product.product_name)
                    .labelled_by(name_label.id);
                // product value
                ui.label("Product Value: ");
                ui.add(egui::DragValue::new(&mut self.cur_product.value).fixed_decimals(2).prefix("R$ "));
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                ui.horizontal_wrapped(|ui| {
                    if ui.button("Add Product").clicked() {
                        let my_product = Product::new(
                            self.cur_index, 
                            self.cur_product.product_name.to_owned(), 
                            self.cur_product.value
                        );
                        self.products.push(my_product);
                    }
                    if ui.button("Save Products").clicked() {
                        let products = products_to_string(&self.products);
                        write("./products.json", &products).expect("Unable to write file");
                    }
                    if ui.button("Read Products").clicked() {
                        let data = read_to_string("./products.json").expect("Unable to read file");
                        self.products = products_from_string(&data);
                    }
                    if ui.button("Search Products").clicked() {
                        if !self.search.is_empty() && !self.searching {
                            self.old_products = self.products.clone();
                            self.products.retain(|p| p.product_name.contains(&self.search));
                            self.searching = true;
                        } else if !self.search.is_empty() && self.searching {
                            self.products = self.old_products.clone();
                            self.old_products = self.products.clone();
                            self.products.retain(|p| p.product_name.contains(&self.search));
                        } else if self.search.is_empty() && self.searching {
                            self.products = self.old_products.clone();
                            self.old_products.clear();
                            self.searching = false;
                        }
                    }
                    ui.text_edit_singleline(&mut self.search);
                });
                ui.add_space(10.0);
                ui.separator();
                ui.add_space(10.0);
                TableBuilder::new(ui)
                .striped(true)
                .column(Column::auto().resizable(true))
                .column(Column::remainder().resizable(true))
                .column(Column::auto().resizable(true))
                .column(Column::auto().resizable(true))
                .header(25.0, |mut header| {
                    header.col(|ui| {
                        ui.heading("ID");
                    });
                    header.col(|ui| {
                        ui.heading("Product Name");
                    });
                    header.col(|ui| {
                        ui.heading("Value");
                    });
                    header.col(|ui| {
                        ui.heading("Delete");
                    });
                })
                .body(|mut body| {
                    let mut index: usize = 0;
                    while index < self.products.len() {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(self.products[index].id.to_string());
                            });
                            row.col(|ui| {
                                ui.text_edit_singleline(&mut self.products[index].product_name);
                            });
                            row.col(|ui| {
                                ui.add(egui::DragValue::new(&mut self.products[index].value));
                            });
                            row.col(|ui| {
                                if ui.button("Delete").clicked() {
                                    self.products.remove(index);
                                }
                            });
                        });
                        index += 1;
                    }
                });

            });
       });
   }
}

fn products_to_string(data: &Vec<Product>) -> String {
    let products_str = serde_json::to_string_pretty(data).expect("Unable to turn Vec into JSON.");
    products_str
}

fn products_from_string(data: &str) -> Vec<Product> {
    let products_vec: Vec<Product> = serde_json::from_str(data).expect("Unable to turn into Vec of Products.");
    products_vec
}