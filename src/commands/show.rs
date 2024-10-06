use clap::ArgMatches;
use prettytable::{format, Attr, Cell, Row, Table};

use crate::{
    helpers::{get_network_validators, get_request_client},
    types::HypermonError,
};

pub async fn show(args: &ArgMatches) -> Result<(), HypermonError> {
    let info_url = args.get_one::<String>("info-url").unwrap().to_string();
    let filter_address = args
        .get_one::<String>("filter-address")
        .unwrap()
        .to_string();

    let client = get_request_client();
    let validators = get_network_validators(&client, info_url).await?;

    let mut table = Table::new();

    // let format = format::FormatBuilder::new()
    //     .column_separator('|')
    //     .borders('|')
    //     .separators(&[format::LinePosition::Top,
    //                   format::LinePosition::Bottom],
    //                 format::LineSeparator::new('-', '+', '+', '+'))
    //     .padding(1, 1)
    //     .build();
    // table.set_format(format);
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);

    table.set_titles(Row::new(vec![
        Cell::new("#").with_style(Attr::Bold),
        Cell::new("📢 Address").with_style(Attr::Bold),
        Cell::new("📓 Name").with_style(Attr::Bold),
        Cell::new("🧱 Recent Blocks").with_style(Attr::Bold),
        Cell::new("🥩 Stake").with_style(Attr::Bold),
        Cell::new("🚨 Is Jailed?").with_style(Attr::Bold),
    ]));

    for (idx, validator) in validators.into_iter().enumerate() {
        table.add_row(validator.as_row(
            idx,
            if validator.validator == filter_address {
                true
            } else {
                false
            },
        ));
    }

    table.printstd();

    Ok(())
}
