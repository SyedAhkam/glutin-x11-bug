use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    ContextBuilder,
};


fn main() {
    let event_loop = EventLoop::new();

    // Works
    //let window_builder = WindowBuilder::new().build(&event_loop);

    // Doesnt work
    let window_builder = WindowBuilder::new();
    let windowed_ctx = ContextBuilder::new()
        .build_windowed(window_builder, &event_loop)
        .expect("failed to create window");
    
    
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::CloseRequested => { *control_flow = ControlFlow::Exit; },
                _ => ()
            },
            _ => ()
        }
    })
}
