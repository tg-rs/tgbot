use crate::types::{
    Contact,
    InputMessageContent,
    InputMessageContentContact,
    InputMessageContentLocation,
    InputMessageContentText,
    InputMessageContentVenue,
    Location,
    Text,
    Venue,
};

#[test]
fn convert() {
    let contact = Contact {
        phone_number: String::from("+79001234567"),
        first_name: String::from("User"),
        last_name: None,
        user_id: None,
        vcard: None,
    };
    let content = InputMessageContent::from(contact.clone());
    assert_eq!(
        content,
        InputMessageContent::Contact(InputMessageContentContact::new(
            contact.phone_number,
            contact.first_name,
        ))
    );

    let location = Location {
        longitude: 0.0,
        latitude: 0.0,
        horizontal_accuracy: None,
        live_period: None,
        heading: None,
        proximity_alert_radius: None,
    };
    let content = InputMessageContent::from(location);
    assert_eq!(
        content,
        InputMessageContent::Location(InputMessageContentLocation::new(location.latitude, location.longitude))
    );

    let content = InputMessageContent::from("text");
    assert_eq!(content, InputMessageContent::Text(InputMessageContentText::new("text")));

    let content = InputMessageContent::from(Text {
        data: String::from("text"),
        entities: None,
    });
    assert_eq!(content, InputMessageContent::Text(InputMessageContentText::new("text")));

    let venue = Venue {
        location,
        title: String::from("Venue"),
        address: String::from("Address"),
        foursquare_id: None,
        foursquare_type: None,
        google_place_id: None,
        google_place_type: None,
    };
    let content = InputMessageContent::from(venue.clone());
    assert_eq!(
        content,
        InputMessageContent::Venue(InputMessageContentVenue::new(
            venue.location.latitude,
            venue.location.longitude,
            venue.title,
            venue.address,
        ))
    );
}
