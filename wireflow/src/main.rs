use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use iced::{stream, window, Application, Element, Size, Subscription, Task};
use iced::futures::{SinkExt, Stream};
use iced::futures::channel::mpsc;
use iced::widget::{button, column, row, text};
use circuit_sim::Simulation;
use circuit_sim::component::{Component, ComponentType};

#[derive(Debug, Clone)]
enum Message {
    START,
    STOP,
    TICK
}

#[derive(Debug, Default)]
struct Wireflow {
    pub simulation: Arc<Mutex<Simulation>>
}

fn main() -> iced::Result {
    iced::application("Wireflow", Wireflow::update, Wireflow::view)
        .centered()
        .window_size((1000.0, 620.0))
        .run_with(|| (Wireflow::demo(), Task::none()))
}

impl Wireflow {
    fn demo() -> Self {
        let simulation = Arc::new(Mutex::new(Simulation::default()));

        {
            let mut simulation = simulation.lock().unwrap();
            simulation.circuit.add(Component::new("clock", ComponentType::CLOCK(
                Instant::now(),
                Duration::from_secs(1)
            )));
        }
        
        Self{simulation}
    }
    pub fn view(&self) -> Element<'_, Message> {
        let simulation = self.simulation.lock().unwrap();
        let circuit = &simulation.circuit;
        let clock = circuit.get("clock").unwrap();
 
        let a = row![
            button("Start").on_press_maybe(
                if !simulation.running { Some(Message::START) } else { None }
            ),
            button("Stop").on_press_maybe(
                if simulation.running { Some(Message::STOP) } else { None }
            )
        ];
        
        column![
            a,
            text!("Clock: {}", clock.get("output0").unwrap().value),
        ].into()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::START => {
                let simulation = self.simulation.clone();
                {
                    simulation.lock().unwrap().running = true;
                }
                
                Task::stream(stream::channel(100, async move |mut sender|{
                    loop{
                        let simulation = simulation.clone();
                        let _ = thread::spawn(move || {
                            {
                                let mut simulation = simulation.lock().unwrap();

                                if !simulation.running {
                                    return;
                                }

                                Simulation::step(&mut simulation.circuit);
                            }
                        }).join();
                        
                        let _ = sender.send(Message::TICK).await;

                    }
                }))
            },
            
            Message::STOP => {
                {
                    let mut simulation = self.simulation.lock().unwrap();
                    simulation.running = false;
                }
                Task::none()
            },
            
            Message::TICK => {
                Task::none()
            }
        }
    }
}
