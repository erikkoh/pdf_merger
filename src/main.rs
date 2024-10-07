use druid::{AppLauncher, Data, Widget, WindowDesc, Env, Lens,PlatformError};
use druid::widget::{Label,Button, Flex};

use rfd:: FileDialog;

use std::path::PathBuf;
use std::vec::Vec;
use std::sync::Arc;

fn main()->Result<(),PlatformError>{
    let main_window = WindowDesc::new(build_app())
    .title("PDF Merger")
    .window_size((400.0, 400.0));

    let inital_state = PDFmerger{
        pdf_name: "Pdf_name".into(),
        file_selction_state: "0 files selected".into(),
        file_number: 0,
        pdf_list: Arc::new(Vec::new()),
    };

    AppLauncher::with_window(main_window)
    .launch(inital_state)?;
    Ok(())

}

#[derive(Clone, Data, Lens)]
struct PDFmerger{
    pdf_name: String,
    file_selction_state: String,
    file_number: i32,
    pdf_list: Arc<Vec<PathBuf>>,
}

fn build_app()-> impl Widget<PDFmerger>{
    
    let mut col = Flex::column();
    col.add_child(Label::new(|data: &PDFmerger, _env: &Env| format!("Hello {}!", data.pdf_name)));
    col.add_child(Button::new("Select files!").on_click(|_ctx, data: &mut PDFmerger, _env|{
        let files = file_explorer();
        match files {
            Some(files) => {
                let files_selected = files.len();

                println!("Number of files selected {files_selected}");
                if data.pdf_list.len() == 0 {
                    println!("5");
                }
                else{
                    let mut files_refarence = &files;
                    for paths in files{
                        println!("5");
                    }
                }
            }
            None =>{
                println!("Failed to select a file");
            }

        }
    }));
    col
}


fn file_explorer() -> Option<Vec<PathBuf>>{
    let file = FileDialog::new()
    .add_filter("pdf",&["pdf"] )
    .set_directory("/")
    .pick_files();
    file
}

// fn merge_pdfs(Vec<PathBuf>) -> Files{

// }