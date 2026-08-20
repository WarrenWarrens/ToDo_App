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
            let mut task_to_delete: Option<usize> = None;

            for (index, task) in self.tasks.iter_mut().enumerate(){
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

            if let Some(index) = task_to_delete {
                self.tasks.remove(index);
            }

        });
    }
}



fn main() -> eframe::Result<()> {
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
