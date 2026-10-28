# Utils

all I have now is this timer thing based off Clear Codes python timer.

## Use case

```rs
let mut my_timer = Timer::new(
    &rl,
    2.0,                  // duration in seconds
    true,                 // repeat
    true,                 // autostart
    Some(Box::new(|| {    // callback function
        println!("Timer triggered!");
    }))
);

while !rl.window_should_close() {
    my_timer.update(&rl);

}
```
