use std::collections::HashMap;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io::{self, Write };


fn main() -> io::Result<()> {
	let mut depts: HashMap::<String, Vec<String>> = HashMap::new();
	let mut select_dept:Option<String> = None; 
	main_menu();
	loop {
		if let Ok(Event::Key(key_event)) = event::read() {
			if key_event.kind == KeyEventKind::Press { 
          match key_event.code {
          	KeyCode::Char(c) => {
							if select_dept.is_none() {
								match c {
									'1' => {
										let input= input_dept("Введите название нового отдела: ")?;
										let eploers: Vec<String> = Vec::new();
										depts.insert(input, eploers);
									}
									'2' => {		
										println!("{depts:?}");		 
										}
									'3' => {
										let input= input_dept("Введите название отдела для удаления: ")?;
										if let Some(dep_name) = depts.remove(&input) {
											println!("Отдел удален");
										} else  {
											println!("Такого отдела нет");
										}
									}
									'4' => {
										select_dept= input_dept("Введите название выбираемого отдела: ").ok();
										
									}
									_ => println!("Нажата другая клавиша: {:?}", key_event.code),
								}
								
							} else {
								
								match c {
									'1' => {
										let input= input_dept("Введите ФИО нового сотрудника: ")?;
										if let Some(vector_dept) = depts.get_mut(select_dept.as_deref().unwrap_or("")) {
        							vector_dept.push(String::from(input)); 
    								} else {
        						println!("Ключ не найден!");
										}
									}
									'2' => {
										println!("{depts:?}");	
									}
									'3' => {
											if let Some(vector_dept) = depts.get_mut(select_dept.as_deref().unwrap_or("")) {
        								for (i, employee) in &mut vector_dept.iter().enumerate() {
        									println!("Индекс: {}, Сотрудник: {}", i, employee);
    										}
												let input= input_dept("Введите номер сотрудника: ")?;
													match input.parse::<usize>() {
    												Ok(num) => {
															if num < vector_dept.len() {
																vector_dept.remove(num);
															} else {
																println!("Нет сотрудника с номером {num}");
															}
															
														},
    												Err(e) => println!("Ошибка парсинга: {}", e),
													}

    								} else {
        						println!("Ключ не найден!");
										}
										
									}
									'4' => {
										select_dept = None;
									}
									_ => println!("Нажата другая клавиша: {:?}", key_event.code),
								}
							}
							
						}
						KeyCode::Enter => {
							if select_dept.is_none() {
              main_menu();
							} else {
								user_menu(select_dept.as_deref().unwrap_or(""));
							}
            }
            KeyCode::Esc => {
              println!("Выход из программы.");
                break;
            }
            _ => println!("Нажата другая клавиша: {:?}", key_event.code),
          }
						//print_menu();
        }
			}

	}

	println!("Count End");
	Ok(())
}

fn input_dept(text: &str) -> io::Result<String> {
	print!("{text}");
  io::stdout().flush()?;
	let mut input = String::new();
  io::stdin().read_line(&mut input)?;
	let str_in = input.trim().to_string();										

	Ok(str_in)
}
fn main_menu() {
	print!("{}[2J{}[1;1H", 27 as char, 27 as char);
	println!("Меню:
	1. Создать новый отдел
	2. Отобразить список отделов
	3. Удалить отдел
	4. Выбрать отдел
	Нажимайте клавиши (Esc для выхода)...");

}

fn user_menu(dep: &str) {
	print!("{}[2J{}[1;1H", 27 as char, 27 as char);
	println!("Выбран отдел {}
	Меню:
	1. Создать нового сотрудника
	2. Отобразить список сотрудников
	3. Удалить сотрудника
	4. Вернуться в основное меню
	Нажимайте клавиши (Esc для выхода)...", dep);

}
