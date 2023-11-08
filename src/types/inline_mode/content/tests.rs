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
    let contact = Contact::new("User", "+79001234567");
    let content = InputMessageContent::from(contact.clone());
    assert_eq!(
        content,
        InputMessageContent::Contact(InputMessageContentContact::new(
            contact.first_name,
            contact.phone_number,
        ))
    );

    let location = Location::new(0.0, 0.0);
    let content = InputMessageContent::from(location);
    assert_eq!(
        content,
        InputMessageContent::Location(InputMessageContentLocation::new(location.latitude, location.longitude))
    );

    let content = InputMessageContent::from("text");
    assert_eq!(content, InputMessageContent::Text(InputMessageContentText::new("text")));

    let content = InputMessageContent::from(Text::from("text"));
    assert_eq!(content, InputMessageContent::Text(InputMessageContentText::new("text")));

    let venue = Venue::new("Venue", "Address", location);
    let content = InputMessageContent::from(venue.clone());
    assert_eq!(
        content,
        InputMessageContent::Venue(InputMessageContentVenue::new(
            venue.address,
            venue.location.latitude,
            venue.location.longitude,
            venue.title,
        ))
    );
}
