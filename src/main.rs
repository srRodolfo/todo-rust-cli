use serde::{Deserialize, Serialize};
use std::fs::{File, read_to_string};
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

fn save_tasks(tasks: &Vec<Task>) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = File::create("tasks.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

fn load_tasks() -> Vec<Task> {
    if let Ok(data) = read_to_string("tasks.json") {
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    }
}

fn main() {
    println!("\n=== To-Do CLI ===");

    let mut tasks: Vec<Task> = load_tasks();

    loop {
        println!("\n--- Opções Disponíveis ---");
        println!("1 - Adicionar tarefa");
        println!("2 - Listar tarefas");
        println!("3 - Concluir/Desmarcar tarefas");
        println!("4 - Remover tarefa");
        println!("5 - Sair\n");
        print!("Informe uma opção: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Erro ao ler entradas");

        match input.trim() {
            "1" => {
                print!("Digite a descrição da tarefa: ");
                io::stdout().flush().unwrap();
                let mut desc = String::new();
                io::stdin().read_line(&mut desc).unwrap();

                let task = Task {
                    id: tasks.len() + 1,
                    description: desc.trim().to_string(),
                    done: false,
                };

                tasks.push(task);
                println!("Tarefa adicionada!");
                save_tasks(&tasks);
            }
            "2" => {
                println!("\n--- Suas tarefas ---");
                if tasks.is_empty() {
                    println!("Nenhuma tarefa encontrada!");
                } else {
                    for t in &tasks {
                        let status = if t.done { "[x]" } else { "[ ]" };
                        println!("{} {} - {}", t.id, status, t.description);
                    }
                }
            }
            "3" => {
                if tasks.is_empty() {
                    println!("Nenhuma tarefa para concluir!");
                } else {
                    print!("\nDigite o ID da tarefa que deseja marcar/demarcar: ");
                    io::stdout().flush().unwrap();
                    let mut id_input = String::new();
                    io::stdin().read_line(&mut id_input).unwrap();

                    if let Ok(id) = id_input.trim().parse::<usize>() {
                        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                            if task.done {
                                print!("Tarefa {} está concluída. Deseja desmarcar? (s/n): ", id);
                                io::stdout().flush().unwrap();
                                let mut response = String::new();
                                io::stdin().read_line(&mut response).unwrap();
                                if response.trim().eq_ignore_ascii_case("s") {
                                    task.done = false;
                                    println!("Tarefa {} desmarcada.", id);
                                } else {
                                    println!("Tarefa permanece concluída.");
                                }
                            } else {
                                task.done = true;
                                println!("Tarefa {} marcada como concluída!", id);
                            }
                        } else {
                            println!("Tarefa com ID {} não encontrada!", id);
                        }
                    } else {
                        println!("Por favor, digite um número válido")
                    }
                    save_tasks(&tasks);
                }
            }
            "4" => {
                if tasks.is_empty() {
                    println!("Nenhuma tarefa para removar!");
                } else {
                    print!("\nDigite o Id da tarefa que deseja remover: ");
                    io::stdout().flush().unwrap();
                    let mut id_input = String::new();
                    io::stdin().read_line(&mut id_input).unwrap();

                    if let Ok(id) = id_input.trim().parse::<usize>() {
                        if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                            tasks.remove(pos);
                            println!("Tarefa {} removida com sucesso!", id);
                        } else {
                            println!("Tarefa com ID {} não encontrada!", id);
                        }
                    } else {
                        println!("Por favor, digite um número válido!");
                    }
                }
                save_tasks(&tasks);
            }
            "5" => {
                println!("Saindo...");
                save_tasks(&tasks);
                break;
            }
            _ => println!("Opção inválida"),
        }
    }
}
