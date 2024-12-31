use std::collections::BinaryHeap;
use std::cmp::Ordering;

// Struktur untuk mewakili tugas dengan prioritas
#[derive(Eq, PartialEq)]
struct Task {
    name: String,
    priority: u8,
    description: String,
}

// Implementasi Ord untuk Task agar bisa diurutkan di BinaryHeap
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Membalik urutan agar prioritas lebih tinggi diproses lebih dahulu
        other.priority.cmp(&self.priority)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Status robot
enum RobotState {
    Idle,
    Busy,
}

// Struktur untuk Robot
struct Robot {
    name: String,
    state: RobotState,
    task_queue: BinaryHeap<Task>,
}

impl Robot {
    // Inisialisasi robot baru
    fn new(name: &str) -> Self {
        Robot {
            name: name.to_string(),
            state: RobotState::Idle,
            task_queue: BinaryHeap::new(),
        }
    }

    // Menambahkan tugas ke antrean
    fn add_task(&mut self, name: &str, priority: u8, description: &str) {
        self.task_queue.push(Task {
            name: name.to_string(),
            priority,
            description: description.to_string(),
        });
        println!("Tugas '{}' dengan prioritas {} ditambahkan ke antrean.", name, priority);
    }

    // Memproses tugas dari antrean
    fn process_tasks(&mut self) {
        while let Some(task) = self.task_queue.pop() {
            self.state = RobotState::Busy;
            println!(
                "🚀 Robot '{}' sedang memproses tugas: '{}' (Prioritas: {}) - {}",
                self.name, task.name, task.priority, task.description
            );
            // Simulasi waktu pemrosesan
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        self.state = RobotState::Idle;
        println!("✅ Semua tugas selesai. Robot '{}' kembali ke status Idle.", self.name);
    }
}

// Fungsi utama
fn main() {
    let mut robot = Robot::new("Atlas");

    // Menambahkan beberapa tugas
    robot.add_task("Angkut Barang", 2, "Mengangkut barang ke gudang.");
    robot.add_task("Isi Ulang Baterai", 1, "Mengisi ulang baterai robot.");
    robot.add_task("Perbaikan Mesin", 3, "Memperbaiki mesin produksi.");

    // Memproses tugas berdasarkan prioritas
    robot.process_tasks();
}
