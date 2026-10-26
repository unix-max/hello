use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::collections::HashMap;
use std::io::{self, Write};

type DeptsMap = HashMap<String, Vec<String>>;

fn main() -> io::Result<()> {
    let mut depts: DeptsMap = HashMap::new();
    let mut select_dept: Option<String> = None;

    show_main_menu();

    loop {
        if let Ok(Event::Key(key_event)) = event::read() {
            // Обрабатываем только событие нажатия (Press), игнорируя Release
            if key_event.kind != KeyEventKind::Press {
                continue;
            }

            match key_event.code {
                KeyCode::Char(c) => {
                    // Используем pattern matching вместо if select_dept.is_none()
                    match &mut select_dept {
                        None => handle_main_menu_click(c, &mut depts, &mut select_dept)?,
                        Some(dept_name) => handle_dept_menu_click(c, &mut depts, dept_name, &mut select_dept)?,
                    }
                }
                KeyCode::Enter => {
                    match &select_dept {
                        None => show_main_menu(),
                        Some(dept) => show_user_menu(dept),
                    }
                }
                KeyCode::Esc => {
                    println!("\nВыход из программы.");
                    break;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

// Логика главного меню
fn handle_main_menu_click(c: char, depts: &mut DeptsMap, select_dept: &mut Option<String>) -> io::Result<()> {
    match c {
        '1' => {
            let input = read_input("\nВведите название нового отдела: ")?;
            if !input.is_empty() {
                depts.entry(input).or_default();
                println!("Отдел создан. Нажмите Enter для обновления меню.");
            }
        }
        '2' => {
            println!("\nСписок отделов и сотрудников:\n{:#?}", depts);
        }
        '3' => {
            let input = read_input("\nВведите название отдела для удаления: ")?;
            if depts.remove(&input).is_some() {
                println!("Отдел удален.");
            } else {
                println!("Такого отдела нет.");
            }
        }
        '4' => {
            let input = read_input("\nВведите название выбираемого отдела: ")?;
            if depts.contains_key(&input) {
                *select_dept = Some(input);
                println!("Отдел выбран. Нажмите Enter для перехода в меню отдела.");
            } else {
                println!("Отдел не найден.");
            }
        }
        _ => println!("\nНеизвестная команда: {}", c),
    }
    Ok(())
}

// Логика меню внутри конкретного отдела
fn handle_dept_menu_click(c: char, depts: &mut DeptsMap, current_dept: &str, select_dept: &mut Option<String>) -> io::Result<()> {
    // Получаем безопасную изменяемую ссылку на вектор сотрудников отдела
    let employees = match depts.get_mut(current_dept) {
        Some(v) => v,
        None => {
            println!("\nОшибка: Текущий отдел был удален!");
            *select_dept = None;
            return Ok(());
        }
    };

    match c {
        '1' => {
            let input = read_input("\nВведите ФИО нового сотрудника: ")?;
            if !input.is_empty() {
                employees.push(input);
                println!("Сотрудник добавлен.");
            }
        }
        '2' => {
            println!("\nСотрудники отдела {}: {:?}", current_dept, employees);
        }
        '3' => {
            if employees.is_empty() {
                println!("\nВ отделе нет сотрудников.");
                return Ok(());
            }

            println!("\nСписок сотрудников:");
            for (i, employee) in employees.iter().enumerate() {
                println!("{}. {}", i, employee);
            }

            let input = read_input("Введите номер сотрудника для удаления: ")?;
            match input.parse::<usize>() {
                Ok(num) if num < employees.len() => {
                    employees.remove(num);
                    println!("Сотрудник удален.");
                }
                _ => println!("Неверный номер сотрудника."),
            }
        }
        '4' => {
            *select_dept = None;
            show_main_menu();
        }
        _ => println!("\nНеизвестная команда: {}", c),
    }
    Ok(())
}

// Универсальная функция ввода строки
fn read_input(prompt: &str) -> io::Result<String> {
    print!("{}", prompt);
    io::stdout().flush()?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

// Очистка экрана вынесена в макрос/функцию для избавления от дублирования кода
fn clear_screen() {
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
}

fn show_main_menu() {
    clear_screen();
    println!(
        "Основное меню:
1. Создать новый отдел
2. Отобразить список отделов
3. Удалить отдел
4. Выбрать отдел
Нажимайте клавиши (Esc для выхода)..."
    );
}

fn show_user_menu(dep: &str) {
    clear_screen();
    println!(
        "Выбран отдел: {}
Меню отдела:
1. Создать нового сотрудника
2. Отобразить список сотрудников
3. Удалить сотрудника
4. Вернуться в основное меню
Нажимайте клавиши (Esc для выхода)...",
        dep
    );
}