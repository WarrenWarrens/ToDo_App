use eframe::egui;


struct Task{
    name: String,
    is_done: bool,
}


#[derive(Default)]
struct TodoApp{
    tasks: Vec<Task>,
    new_task_input: String,
}

impl eframe::App for TodoApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Rust To-Do List");
            ui.add_space(10.0);

            ui.horizontal(|ui|{
                ui.text_edit_singleline(&mut self.new_task_input);

                if ui.button("Add Task").clicked() && !self.new_task_input.trim().is_empty() {
                    self.tasks.push(Task {
                        name: self.new_task_input.trim().to_string(),
                        is_done: false,
                    });
                    self.new_task_input.clear();
                }
            });

            ui.separator();
            let row_height: f32 = 24.0;
            let total_tasks: usize = self.tasks.len();
            let mut task_to_delete: Option<usize> = None;

            egui::ScrollArea::vertical().show_rows(ui, row_height, total_tasks, |ui, row_range|{

                for index in row_range{
                    let task = &mut self.tasks[index];

                    ui.horizontal(|ui|{
                        ui.checkbox(&mut task.is_done, "");

                        if task.is_done {
                            ui.label(egui::RichText::new(&task.name).strikethrough());

                        }
                        else {
                            ui.label(&task.name);
                        }

                        if ui.button("Delete").clicked(){
                            task_to_delete = Some(index);
                        }
                    });
                }

            });


            if let Some(index) = task_to_delete {
                self.tasks.remove(index);
            }

        });
    }
}



fn main() -> eframe::Result<()> {
    let icon_bytes = include_bytes!("../assets/todosprite.png");

    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load image")
        .to_rgba8();

    let (width, height) = image.dimensions();
    let rgba_data = image.into_raw();

    let icon = std::sync::Arc::new(egui::IconData {
        rgba: rgba_data,
        width,
        height
    });

    let _options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_icon(icon),
        ..Default::default()
    };

    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0,500.0]),
        ..Default::default()

    };

    eframe::run_native(
        "To-Do App",
        options,
        Box::new(|_cc| Box::new(TodoApp::default())),
    )
}
