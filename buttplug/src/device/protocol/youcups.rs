use crate::create_buttplug_protocol;

create_buttplug_protocol!(Youcups,
    (VibrateCmd, {
        // TODO Convert to using generic command manager
        let msg = DeviceWriteCmd::new(
            Endpoint::Tx,
            format!("$SYS,{}?", (msg.speeds[0].speed * 8.0) as u8)
                .as_bytes()
                .to_vec(),
            false,
        );
        device.write_value(msg.into()).await?;
        Ok(ButtplugMessageUnion::Ok(messages::Ok::default()))
    })
);

// TODO Write Tests