use serde::{Deserialize, Serialize};  // importa as traits de serialização e desserialização (conversão entre structs e JSON, por exemplo)
use std::fs::{File, read_to_string}; // importa File para criar/abrir arquivos e read_to_string para ler o conteúdo de um arquivo inteiro como String
use std::io::{self, Write}; // importa a biblioteca de entrada/saída padrão; 'Write' é usado para poder usar stdout().flush()

#[derive(Debug, Serialize, Deserialize)] // gera implementações automáticas para Debug (impressão formatada), Serialize e Deserialize (conversão para/ de JSON)
struct Task { // define uma struct chamada Task (tarefa)
    id: usize, // identificador único da tarefa (usize = tamanho dependente da arquitetura da máquina)
    description: String, // descrição textual da tarefa
    done: bool, // indica se a tarefa já foi concluída (true) ou não (false)
}

fn save_tasks(tasks: &Vec<Task>) { // função que recebe uma referência para um vetor de Task e salva em arquivo
    let json = serde_json::to_string_pretty(tasks).unwrap(); // converte a lista de tarefas para uma String em formato JSON "bonito" (indentado)
    let mut file = File::create("tasks.json").unwrap(); // cria (ou sobrescreve) o arquivo chamado "tasks.json"
    file.write_all(json.as_bytes()).unwrap(); // escreve o conteúdo JSON no arquivo como bytes
}

fn load_tasks() -> Vec<Task> { // função que carrega as tarefas do arquivo e retorna um vetor de Task
    if let Ok(data) = read_to_string("tasks.json") { // tenta ler o conteúdo do arquivo "tasks.json"; se der certo, guarda em 'data'
        serde_json::from_str(&data).unwrap_or_else(|_| Vec::new()) // tenta converter o JSON em vetor de Task; se falhar, retorna vetor vazio
    } else {
        Vec::new() // ...retorna um vetor vazio
    }
}

fn main() {
    println!("\n=== To-Do CLI ===");

    let mut tasks: Vec<Task> = load_tasks(); // cria uma variável mutável 'tasks' do tipo vetor de Task e inicializa com o que foi carregado do arquivo JSON

    loop {
        println!("\n--- Opções Disponíveis ---");
        println!("1 - Adicionar tarefa");
        println!("2 - Listar tarefas");
        println!("3 - Concluir/Desmarcar tarefas");
        println!("4 - Remover tarefa");
        println!("5 - Sair\n");
        print!("Informe uma opção: ");
        io::stdout().flush().unwrap(); // força a saída do buffer do terminal, garantindo que todo o texto escrito com print! apareça imediatamente na tela

        let mut input = String::new(); // cria uma String mutável vazia para armazenar o que o usuário digitar
        io::stdin() // acessa a entrada padrão do terminal (teclado)
            .read_line(&mut input) // lê uma linha digitada pelo usuário e armazena em 'input'
            .expect("Erro ao ler entradas"); // trata possíveis erros: se ocorrer, exibe a mensagem e encerra o programa

        match input.trim() { // remove espaços em branco e quebras de linha do começo/fim da entrada do usuário e compara o valor para decidir ações diferentes
            "1" => {
                print!("Digite a descrição da tarefa: ");
                io::stdout().flush().unwrap();

                let mut desc = String::new(); // cria uma String mutável vazia para armazenar a descrição da nova tarefa
                io::stdin().read_line(&mut desc).unwrap(); // lê uma linha digitada pelo usuário e armazena em 'desc'; unwrap() faz o programa entrar em pânico caso ocorra algum erro

                let task = Task { // cria uma nova instância de Task
                    id: tasks.len() + 1, // define o id como o tamanho atual do vetor + 1 (garante um id único sequencial)
                    description: desc.trim().to_string(), // remove espaços e quebras de linha da descrição e converte para String
                    done: false, // define a tarefa como não concluída inicialmente
                };

                tasks.push(task); // adiciona a nova tarefa criada ao vetor 'tasks', atualizando a lista em memória
                println!("Tarefa adicionada!");
                save_tasks(&tasks); // chama a função para salvar todas as tarefas do vetor 'tasks' no arquivo "tasks.json", persistindo as alterações
            }
            "2" => {
                println!("\n--- Suas tarefas ---");
                if tasks.is_empty() {
                    println!("Nenhuma tarefa encontrada!");
                } else {
                    for t in &tasks { // percorre cada tarefa no vetor 'tasks' usando referência para não copiar
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

                    let mut id_input = String::new(); // cria uma String mutável vazia para armazenar o id digitado pelo usuário
                    io::stdin().read_line(&mut id_input).unwrap(); // lê uma linha do teclado e armazena em 'id_input'; unwrap() causa pânico se houver erro

                    if let Ok(id) = id_input.trim().parse::<usize>() { // tenta converter a entrada do usuário em um número inteiro (usize); ignora espaços com trim()
                        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) { // procura no vetor 'tasks' a tarefa com o id informado e retorna uma referência mutável para ela, se existir
                            if task.done {
                                print!("Tarefa {} está concluída. Deseja desmarcar? (s/n): ", id);
                                io::stdout().flush().unwrap();

                                let mut response = String::new(); // cria uma String mutável para armazenar a resposta do usuário
                                io::stdin().read_line(&mut response).unwrap(); // lê a linha digitada pelo usuário e armazena em 'response'; unwrap() causa pânico em caso de erro

                                if response.trim().eq_ignore_ascii_case("s") { // remove espaços e quebras de linha com trim() e compara ignorando maiúsculas/minúsculas; verifica se o usuário respondeu "s" (sim)
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

                    let mut id_input = String::new(); // cria uma String mutável para armazenar o id da tarefa que o usuário deseja remover
                    io::stdin().read_line(&mut id_input).unwrap(); // lê uma linha do teclado e armazena em 'id_input'; unwrap() faz o programa entrar em pânico se ocorrer erro

                    if let Ok(id) = id_input.trim().parse::<usize>() { // tenta converter a entrada do usuário em um número inteiro (usize); ignora espaços com trim()
                        if let Some(pos) = tasks.iter().position(|t| t.id == id) { // procura a posição da tarefa no vetor cujo id corresponda ao número informado
                            tasks.remove(pos); // remove a tarefa encontrada do vetor 'tasks' usando o índice 'pos'
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
