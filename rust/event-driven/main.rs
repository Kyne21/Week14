use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

// Enum untuk berbagai jenis event
enum Event {
    ObstacleDetected(String), // Event ketika ada rintangan baru
    GoalChanged(String),      // Event ketika tujuan berubah
    Idle,                     // Event ketika robot tidak ada aktivitas
}

// Struktur untuk Robot
struct Robot {
    name: String,
}

impl Robot {
    fn new(name: &str) -> Self {
        Robot {
            name: name.to_string(),
        }
    }

    // Fungsi untuk menangani event
    fn handle_event(&self, event: Event) {
        match event {
            Event::ObstacleDetected(obstacle) => {
                println!(
                    "🚧 Robot '{}' mendeteksi rintangan: '{}'. Menghitung rute alternatif...",
                    self.name, obstacle
                );
                // Logika menghindari rintangan
                self.avoid_obstacle();
            }
            Event::GoalChanged(new_goal) => {
                println!(
                    "🎯 Robot '{}' memiliki tujuan baru: '{}'. Memulai navigasi...",
                    self.name, new_goal
                );
                // Logika menuju tujuan baru
                self.navigate_to(&new_goal);
            }
            Event::Idle => {
                println!("⏳ Robot '{}' sedang dalam keadaan idle.", self.name);
            }
        }
    }

    fn avoid_obstacle(&self) {
        println!("🤖 Robot '{}' menghindari rintangan...", self.name);
        thread::sleep(Duration::from_secs(1));
        println!("✅ Rintangan berhasil dihindari.");
    }

    fn navigate_to(&self, goal: &str) {
        println!("🛤️ Robot '{}' bergerak menuju tujuan: '{}'", self.name, goal);
        thread::sleep(Duration::from_secs(2));
        println!("🏁 Robot '{}' telah mencapai tujuan: '{}'", self.name, goal);
    }
}

// Fungsi untuk memicu event secara acak (simulasi sensor lingkungan)
fn environment_simulation(sender: Sender<Event>) {
    let events = vec![
        Event::ObstacleDetected(String::from("Batu Besar")),
        Event::GoalChanged(String::from("Pos A")),
        Event::Idle,
        Event::GoalChanged(String::from("Pos B")),
        Event::ObstacleDetected(String::from("Parit Lebar")),
    ];

    for event in events {
        println!("\n🌍 Event baru terdeteksi di lingkungan.");
        sender.send(event).expect("Gagal mengirim event");
        thread::sleep(Duration::from_secs(3));
    }
}

// Fungsi utama
fn main() {
    // Membuat saluran komunikasi untuk event
    let (tx, rx): (Sender<Event>, Receiver<Event>) = mpsc::channel();

    let robot = Robot::new("Atlas");

    // Simulasi lingkungan berjalan di thread terpisah
    let env_thread = thread::spawn(move || {
        environment_simulation(tx);
    });

    // Event loop untuk menangani event
    for event in rx {
        robot.handle_event(event);
    }

    env_thread.join().expect("Thread simulasi lingkungan gagal selesai");
}
