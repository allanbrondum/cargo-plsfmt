fn test() {
    println!("{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test");

    trace!(test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", test = %"test", "{} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test", "test");

    test!(test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test test );

    list!(if a {b} else {c}, if a {b} else {c});
    list!(1234, 1234, 1234, 1234, 1234);
    list!(1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, );
    list!(
        if a {b} else {c},
        if a {b} else {c},
    );
    list!(if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, );
    list!(1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, );
    list!(fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, );

    list![if a {b} else {c}, if a {b} else {c}];
    list![1234, 1234, 1234, 1234, 1234];
    let vec = vec![
        1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, ];
    list![
        if a {b} else {c},
        if a {b} else {c},
    ];
    list![if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, if a {b} else {c}, ];
    list![1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, 1234, ];
    list![fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, fn a() {}, ];

    vec![if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c}; 4];

    vec![if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c}, if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c}, if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c}];

    vec!(if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c}, if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c}, if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c} + if a {b} else {c});

    let d = if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c } + if a { b } else { c };

    {
        let mut items = SmallVec::new();
        items.push(stream_item
            .context("market event stream ended")?
            .context("market event stream error")?);

        let batch_until = Instant::now() + Duration::from_millis(1);
        while let Ok(stream_item) = tokio::time::timeout_at(batch_until, stream.next()).await {
            items.push(stream_item
                .context("market event stream ended")?
                .context("market event stream error")?);
        }

        market_event_tx.send(items)?;
    }

    let a = 0;
    match a {
        1 => (),
        _ => (),
    }

}