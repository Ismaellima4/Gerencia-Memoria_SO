#![allow(unused)]

// Estrutura para representar um quadro na memória física
struct Frame {
    //Número de quadro
    number: usize,
    // Página alocada neste quadro (opcional)
    page: Option<usize>,
}

// Estrutura para representar uma entrada na tabela de página
struct PageTableEntry {
    // Número do quadro físico correspondente
    frame: Option<usize>,
    // Bit de presença (true se a página está na memória)
    present: bool,
}

// Estrutura para representar um processo
struct Process {
    // ID do processo
    id: usize,
    // Tabela de páginas do processo
    page_table: Vec<PageTableEntry>,
}

// Inicializa a memória
fn init_memory(num_frames: usize) -> Vec<Frame> {
    let memory: Vec<Frame> = (0..num_frames)
        .into_iter()
        .map(|i| Frame {
            number: i,
            page: None,
        })
        .collect();

    memory
}

fn allocate_page(process: &mut Process, page_number: usize, memory: &mut Vec<Frame>) {
    // 1. Encontrar um quadro livre na memória física
    if let Some(frame) = memory.iter_mut().find(|f| f.page.is_none()) {
        // 2. Atualizar a tabela de páginas do processo
        process.page_table[page_number] = PageTableEntry {
            frame: Some(frame.number),
            present: true,
        };
        // 3. Atualizar o quadro na memória física
        frame.page = Some(page_number);
    } else {
        // Se não houver quadros livres (messagem de error simples)
        println!("Memória física cheia");
    }
}

fn replace_page(process: &mut Process, page_number: usize, memory: &mut Vec<Frame>) {
    // 1. Encontrar a página mais antiga na memória física (usando um contador ou timestamp)
    let oldest_page = memory.iter().find(|f| f.page.is_some()).unwrap();
    // 2. Obter o número da página e o processo ao qual ela pertence
    let oldest_page_number = oldest_page.page.unwrap();
    // 3. Atualizar a tabela de páginas do processo da página mais antiga
    // (definir o bit de presença como false)
    // 4. Atualizar a tabela de páginas do processo da nova página
    process.page_table[page_number] = PageTableEntry {
        frame: Some(oldest_page.number),
        present: true,
    };
    // 5. Atualizar o quadro na memória física
    let frame = memory
        .iter_mut()
        .find(|f| f.number == oldest_page_number)
        .unwrap();
    frame.page = Some(page_number);
}

fn print_memory_state(memory: &Vec<Frame>, processes: &Vec<Process>) {
    println!("Memória Física:");
    for frame in memory {
        println!("Quadro {}: {:?}", frame.number, frame.page);
    }

    println!("\nTabelas de Páginas:");
    for process in processes {
        println!("Processo {}:", process.id);
        for (i, entry) in process.page_table.iter().enumerate() {
            println!("Página {}: {:?}", i, entry.frame);
        }
    }
}

fn main() {
    // Inicializar a memória física com 10 quadros
    let mut memory = init_memory(10);

    // Criar dois processos
    let mut processes = vec![
        Process {
            id: 1,
            page_table: vec![
                PageTableEntry {
                    frame: None,
                    present: false,
                },
                PageTableEntry {
                    frame: None,
                    present: false,
                },
                PageTableEntry {
                    frame: None,
                    present: false,
                },
            ],
        },
        Process {
            id: 2,
            page_table: vec![
                PageTableEntry {
                    frame: None,
                    present: false,
                },
                PageTableEntry {
                    frame: None,
                    present: false,
                },
            ],
        },
    ];

    // Alocar páginas para os processos
    allocate_page(&mut processes[0], 0, &mut memory);
    allocate_page(&mut processes[0], 1, &mut memory);
    allocate_page(&mut processes[1], 0, &mut memory);

    // Exibir o estado do sistema
    print_memory_state(&memory, &processes);

    // Substituir uma página
    replace_page(&mut processes[0], 2, &mut memory);

    // Exibir o estado do sistema após a substituição
    print_memory_state(&memory, &processes);
}
