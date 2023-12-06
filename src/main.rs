use thirtyfour::prelude::*;
use serde_json::Value;
use tokio;
use tokio::time::Duration;
use crate::tokio::time::sleep;

#[tokio::main]
async fn main() -> WebDriverResult<()> {
    // Setup WebDriver
    let mut caps = DesiredCapabilities::chrome();
    caps.add_chrome_arg("--enable-automation")?;
    let mut driver = WebDriver::new("http://127.0.0.1:9515", &caps).await?;
    driver.set_request_timeout(Duration::from_secs(9999999));

    // Navigate to the page
    driver.get("https://ised-isde.canada.ca/cc/lgcy/fdrlCrpSrch.html").await?;

    // Prepare JSON data for form filling and request
    let json_data = r#"
        {
            "corpName":"Montreal",
            "corpNumber": " ",
            "busNumber": " "
        }
    "#;

    let json_data2 = r#"
        {
            "firstName":"Montreal",
            "lastName": " eddpro",
            "phoneNumber": "23456792 ",
            "emailAddress": "eddpro@gmail.com ",
            "emailAddressConfirm":"eddpro@gmail.com"
        }
    "#;

    let form_data: Value = serde_json::from_str(json_data)?;
    let form_data2: Value = serde_json::from_str(json_data2)?;

    // Fill and submit the initial form
    fill_and_submit_form(&driver, &form_data).await?;

    // Iterate through found records and perform actions
    iterate_and_process_records(&driver, &form_data, &form_data2).await?;

    // Close the browser window
    // driver.quit().await?;

    Ok(())
}

async fn fill_and_submit_form(driver: &WebDriver, form_data: &Value) -> WebDriverResult<()> {
    // Fill Corporate Name field
    let corp_name_field = driver.find_element(By::Id("corpName")).await?;
    corp_name_field.send_keys(form_data["corpName"].as_str().unwrap()).await?;

    // Fill Corporation Number field
    let corp_number_field = driver.find_element(By::Id("corpNumber")).await?;
    corp_number_field.send_keys(form_data["corpNumber"].as_str().unwrap()).await?;

    // Fill Business Number field
    let bus_number_field = driver.find_element(By::Id("busNumber")).await?;
    bus_number_field.send_keys(form_data["busNumber"].as_str().unwrap()).await?;

    // Submit the form
    let search_button = driver.find_element(By::Name("buttonNext")).await?;
    search_button.click().await?;

    Ok(())
}

async fn iterate_and_process_records(driver: &WebDriver, form_data: &Value, form_data2: &Value) -> WebDriverResult<()> {
    // Find all record links on the page
    let record_links = driver.find_elements(By::Css("[title='442911-7']")).await?;

    // Iterate through each record link
    for record_link in record_links {
        // Click on the record link
        record_link.click().await?;

        // Click on "Order Copies of..."
        let order_copies_link = driver.find_element(By::Css("a.btn.btn-primary")).await?;
        order_copies_link.click().await?;
        click_next_button(&driver).await?;

        sleep(Duration::from_secs(5)).await;

        let tr_element = driver.find_element(By::Css("tr th input#selectToggle")).await?;
        // Check the checkbox.
        tr_element.click().await?;

        sleep(Duration::from_secs(5)).await;
        // // Click the button.
        button(&driver).await?;

        // Enter contact information on the next page
        fill_and_submit_form2(&driver, &form_data2).await?;

        // Submit the form on the next page
        let button_element = driver.find_element(By::Css("button.btn.btn-primary[type='submit']")).await?;
        button_element.click().await?;
    }

    Ok(())
}

async fn click_next_button(driver: &WebDriver) -> WebDriverResult<()> {
    // Wait for the Next button to be present
    let next_button_selector = By::Css("button.btn.btn-primary[data-role='none'][data-ng-click='next()']");
    // Find and click the Next button
    let next_button = driver.find_element(next_button_selector).await?;
    next_button.click().await?;

    Ok(())
}

async fn button(driver: &WebDriver) -> WebDriverResult<()> {
    let button = driver.find_element(By::Css("[data-ng-click='next()']")).await?;
    // Click the button.
    button.click().await?;

    Ok(())
}

// Submit contact data
async fn fill_and_submit_form2(driver: &WebDriver, form_data2: &Value) -> WebDriverResult<()> {
    // Fill firstname
    let first_name_field = driver.find_element(By::Id("firstName")).await?;
    first_name_field.send_keys(form_data2["firstName"].as_str().unwrap()).await?;

    // Fill lastname
    let last_name_field = driver.find_element(By::Id("lastName")).await?;
    last_name_field.send_keys(form_data2["lastName"].as_str().unwrap()).await?;

    // Phone number
    let phone_number_field = driver.find_element(By::Id("phoneNumber")).await?;
    phone_number_field.send_keys(form_data2["phoneNumber"].as_str().unwrap()).await?;

    // Fill emailAdress
    let email_address_field = driver.find_element(By::Id("emailAddress")).await?;
    email_address_field.send_keys(form_data2["emailAddress"].as_str().unwrap()).await?;

    // Fill emailAdressConfirm
    let email_conf_field = driver.find_element(By::Id("emailAddressConfirm")).await?;
    email_conf_field.send_keys(form_data2["emailAddressConfirm"].as_str().unwrap()).await?;

    // Submit the form
    let button_element = driver.find_element(By::Css("button.btn.btn-primary[type='submit']")).await?;
    button_element.click().await?;

    Ok(())
}


