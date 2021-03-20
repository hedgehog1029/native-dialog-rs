use std::process::Command;

use crate::{Error, MessageType, Result};
use crate::dialog::{DialogImpl, MessageAlert, MessageConfirm};

use super::{escape_pango_entities, should_use, UseCommand};

impl DialogImpl for MessageAlert<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        let params = Params {
            title: self.title,
            text: self.text,
            typ: self.typ,
            ask: false,
        };

        match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params)?,
            UseCommand::Zenity(cmd) => call_zenity(cmd, params)?,
        };

        Ok(())
    }
}

impl DialogImpl for MessageConfirm<'_> {
    fn show(&mut self) -> Result<Self::Output> {
        let command = should_use().ok_or(Error::NoImplementation)?;

        let params = Params {
            title: self.title,
            text: self.text,
            typ: self.typ,
            ask: true,
        };

        match command {
            UseCommand::KDialog(cmd) => call_kdialog(cmd, params),
            UseCommand::Zenity(cmd) => call_zenity(cmd, params),
        }
    }
}

struct Params<'a> {
    title: &'a str,
    text: &'a str,
    typ: MessageType,
    ask: bool,
}

fn call_kdialog(mut command: Command, params: Params) -> Result<bool> {
    if params.ask {
        command.arg("--yesno");
    } else {
        command.arg("--msgbox");
    }

    command.arg(escape_pango_entities(params.text));

    command.arg("--title");
    command.arg(params.title);

    match params.typ {
        MessageType::Info => command.arg("--icon=dialog-information"),
        MessageType::Warning => command.arg("--icon=dialog-warning"),
        MessageType::Error => command.arg("--icon=dialog-error"),
    };

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(true),
        Some(_) => Ok(false),
        _ => Err(Error::UnexpectedOutput("kdialog")),
    }
}

fn call_zenity(mut command: Command, params: Params) -> Result<bool> {
    command.arg("--width=400");

    if params.ask {
        command.arg("--question");
        match params.typ {
            MessageType::Info => command.arg("--icon-name=dialog-information"),
            MessageType::Warning => command.arg("--icon-name=dialog-warning"),
            MessageType::Error => command.arg("--icon-name=dialog-error"),
        };
    } else {
        match params.typ {
            MessageType::Info => command.arg("--info"),
            MessageType::Warning => command.arg("--warning"),
            MessageType::Error => command.arg("--error"),
        };
    }

    command.arg("--title");
    command.arg(params.title);

    command.arg("--text");
    command.arg(escape_pango_entities(params.text));

    let output = command.output()?;

    match output.status.code() {
        Some(0) => Ok(true),
        Some(_) => Ok(false),
        _ => Err(Error::UnexpectedOutput("zenity")),
    }
}
