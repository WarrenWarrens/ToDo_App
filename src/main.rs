use eframe::egui;
use directories::ProjectDirs;
use std::path::PathBuf;

fn get_save_path() -> PathBuf {
    if let Some(proj_dirs) = ProjectDirs::from("","","TodoAoo")
    {
        let dir = proj_dirs.data_dir();
        std::fs::create_dir_all(dir).unwrap_or_default();
        return dir.join("tasks.json");
    }
    PathBuf::from("tasks.json")
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Task{
    name: String,
    is_done: bool,
}


#[derive(Default)]
struct TodoApp{
    tasks: Vec<Task>,
    new_task_input: String,
    show_exit_dialog: bool,
}

impl eframe::App for TodoApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){

        if ctx.input(|i| i.viewport().close_requested())
        {
            ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
            self.show_exit_dialog = true;
        }

        if self.show_exit_dialog{
            egui::Window::new("Save before quitting?").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    if ui.button("Yes (Save)").clicked()
                    {
                        let save_path = get_save_path();
                        if let Ok(json) = serde_json::to_string_pretty(&self.tasks){
                            let _ = std::fs::write(save_path, json);
                        }
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close)

                    }
                    if ui.button("No (Don't Save)").clicked(){
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);

                    }
                    if ui.button("Cancel (Return)").clicked(){
                        self.show_exit_dialog = false;
                    }

                });
            });
        }

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

                if ui.button("Clear Completed").clicked(){
                    self.tasks.retain(|task| !task.is_done);
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

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])
            .with_icon(icon),
        ..Default::default()
    };

    let save_path = get_save_path();
    let mut starting_tasks = Vec::new();

    if let Ok(json_string) = std::fs::read_to_string(&save_path){
        if let Ok(parsed_tasks) = serde_json::from_str(&json_string)
        {
            starting_tasks = parsed_tasks;
        }
    }

    let app = TodoApp
    {
        tasks: starting_tasks,
        new_task_input: String::new(),
        show_exit_dialog: false,
    };


    eframe::run_native(
        "To-Do App",
        options,
        Box::new(|_cc| Box::new(app)),
    )
}
