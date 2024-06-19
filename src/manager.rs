use std::str::Split;

use crate::html::{main_menu::{MainMenuGroup, Profile}, Htmls};

pub fn edit_group(mut data: Split<&str>, htmls: &mut Htmls) -> Option<()> {
    let main_menu = &mut htmls.main_menu_struct;
    let order = data.next()?.parse::<usize>().ok()?;
    let new_group_name = data.next()?;

    let group = main_menu.0.get_mut(order)?;
    group.name.clear();
    group.name.push_str(new_group_name);

    htmls.update_main_menu();
    Some(())
}

pub fn delete_group(mut data: Split<&str>, htmls: &mut Htmls) -> Option<()> {
    let main_menu = &mut htmls.main_menu_struct;
    let order = data.next()?.parse::<usize>().ok()?;
    let group_name = data.next()?;

    if &main_menu.0.get(order)?.name == group_name {
        main_menu.0.remove(order);

        htmls.update_main_menu();
        Some(())
    } else {
        None
    }
}

pub fn add_group(mut data: Split<&str>, htmls: &mut Htmls) -> Option<()> {
    let main_menu = &mut htmls.main_menu_struct;
    let group_name = data.next()?;
    main_menu.0.push(MainMenuGroup::new(group_name.to_string()));

    htmls.update_main_menu();
    Some(())
}

pub fn edit_profile(mut data: Split<&str>, htmls: &mut Htmls) -> Option<()> {
    let main_menu = &mut htmls.main_menu_struct;
    let group_order = data.next()?.parse::<usize>().ok()?;
    let profile_order = data.next()?.parse::<usize>().ok()?;
    let name = data.next()?;
    let business = data.next()?;
    let phone = data.next()?;
    let email = data.next()?;

    let profile = main_menu.0
        .get_mut(group_order)?
        .profiles
        .get_mut(profile_order)?;

    profile.name.clear();
    profile.name.push_str(name);
    profile.business.clear();
    profile.business.push_str(business);
    profile.phone.clear();
    profile.phone.push_str(phone);
    profile.email.clear();
    profile.email.push_str(email);

    htmls.update_main_menu();
    Some(())
}

pub fn delete_profile(mut data: Split<&str>, htmls: &mut Htmls) -> Option<()> {
    let main_menu = &mut htmls.main_menu_struct;
    let group_order = data.next()?.parse::<usize>().ok()?;
    let profile_order = data.next()?.parse::<usize>().ok()?;
    let name = data.next()?;
    let business = data.next()?;
    let phone = data.next()?;
    let email = data.next()?;

    let group = main_menu.0.get_mut(group_order)?;

    let profile = group.profiles.get(profile_order)?;

    if {
        &profile.name == name &&
        &profile.business == business &&
        &profile.phone == phone &&
        &profile.email == email
    } {
        group.profiles.remove(profile_order);

        htmls.update_main_menu();
        Some(())
    } else {
        None
    }
}

pub fn add_profile(mut data: Split<&str>, htmls: &mut Htmls) -> Option<()> {
    let main_menu = &mut htmls.main_menu_struct;
    let group_order = data.next()?.parse::<usize>().ok()?;
    let _ = data.next()?;
    let name = data.next()?;
    let business = data.next()?;
    let phone = data.next()?;
    let email = data.next()?;

    let group = main_menu.0.get_mut(group_order)?;

    group.profiles.push(Profile {
        name: name.to_string(),
        business: business.to_string(),
        phone: phone.to_string(),
        email: email.to_string(),
        image_path: String::new(),
    });

    htmls.update_main_menu();
    Some(())
}
