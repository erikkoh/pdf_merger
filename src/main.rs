use druid::{widget::Label, AppLauncher, Data, Widget, WindowDesc, Env, Lens,PlatformError};
use druid::widget::{Button, Flex};

use pdfmerger_derived_lenses::file_number;
use rfd:: FileDialog;

use std::path::PathBuf;
use std::vec::Vec;

fn main()->Result<(),PlatformError>{
    let main_window = WindowDesc::new(build_app())
    .title("PDF Merger")
    .window_size((400.0, 400.0));

    let inital_state = PDFmerger{
        pdf_name: "Pdf_name".into(),
        file_selction_state: "0 files selected".into(),
        file_number: 0,
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
}

fn build_app()-> impl Widget<PDFmerger>{
    let mut pdfs_list:Vec<PathBuf>;
    
    
    let mut col = Flex::column();
    col.add_child(Label::new(|data: &PDFmerger, _env: &Env| format!("Hello {}!", data.pdf_name)));
    col.add_child(Button::new("Select files!").on_click(|_ctx,wiget_info:  &mut PDFmerger, _env|{
        let files = file_explorer();
        match files {
            Some(files) => {
                let files_selected = files.len();
                wiget_info.file_number += files_selected as i32;

                println!("Number of files selected {files_selected}")
                if pdfs_list.len() == 0 {
                    pdfs_list = files
                }
                else{
                    pdfs_list.append(&mut files);
                }
            }
            None =>{
                println!("Failed to select a file")
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