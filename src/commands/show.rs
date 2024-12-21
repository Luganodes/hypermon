use clap::ArgMatches;
use prettytable::{format, Attr, Cell, Row, Table};
use tracing::info;

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
    let only_active = args.get_one::<bool>("only-active").unwrap();
    let only_jailed = args.get_one::<bool>("only-jailed").unwrap();

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

    info!("{} - {}", only_active, only_jailed);

    table.set_titles(Row::new(vec![
        Cell::new("#").with_style(Attr::Bold),
        Cell::new("📢 Address").with_style(Attr::Bold),
        Cell::new("📓 Name").with_style(Attr::Bold),
        Cell::new("🧱 Recent Blocks").with_style(Attr::Bold),
        Cell::new("🥩 Stake").with_style(Attr::Bold),
        Cell::new("🚨 Is Jailed?").with_style(Attr::Bold),
    ]));

    for (idx, validator) in validators.into_iter().enumerate() {
        // If any one of the flags is set to true, show accordingly
        if *only_jailed || *only_active {
            if *only_jailed && validator.is_jailed {
                table.add_row(validator.as_row(
                    idx,
                    if validator.validator == filter_address {
                        true
                    } else {
                        false
                    },
                ));
            } else if *only_active && !validator.is_jailed {
                table.add_row(validator.as_row(
                    idx,
                    if validator.validator == filter_address {
                        true
                    } else {
                        false
                    },
                ));
            }
        } else { // Else show all validators
            table.add_row(validator.as_row(
                idx,
                if validator.validator == filter_address {
                    true
                } else {
                    false
                },
            ));
        }
    }

    table.printstd();

    Ok(())
}
