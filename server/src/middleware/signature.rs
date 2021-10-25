use crate::{
  error::{Error, ServiceError},
  global::Client,
  middleware::{
    header_x_client_date::XClientDate, header_x_client_id::XClientId,
    header_x_client_signature::XClientSignature, Locale,
  },
  util::sign::Sign,
};
use axum::{
  async_trait,
  extract::{FromRequest, OriginalUri, RawQuery, RequestParts, TypedHeader},
};
pub struct Signature {
  pub client_id: i64,
}
#[async_trait]
impl<B> FromRequest<B> for Signature
where
  B: Send,
{
  type Rejection = ServiceError;

  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    // get locale
    // check is signature exists
    let TypedHeader(XClientSignature(x_client_signature)) =
      TypedHeader::<XClientSignature>::from_request(req).await?;
    // check client id is exists
    let TypedHeader(XClientId(x_client_id)) = TypedHeader::<XClientId>::from_request(req).await?;
    // check is client date exists
    let TypedHeader(XClientDate(x_client_date)) =
      TypedHeader::<XClientDate>::from_request(req).await?;
    let mut querystring = "".to_string();
    let RawQuery(query_option) = RawQuery::from_request(req).await?;
    if let Some(x) = query_option {
      querystring = x;
    }
    let OriginalUri(original_uri) = OriginalUri::from_request(req).await?;
    let path = original_uri.path();

    let locale = Locale::from_request(req).await?;
    let client_option = Client::get(x_client_id);
    let method = req.method().as_str();

    if let Some(client) = client_option {
      let sign = Sign::new(
        method.to_string(),
        path.to_string(),
        querystring,
        x_client_id.to_string(),
        x_client_date.to_string(),
      );
      sign.verify(x_client_signature, client.client_secret.clone(), &locale)?;
      // Decode the user data
      Ok(Signature {
        client_id: x_client_id,
      })
    } else {
      return Err(ServiceError::client_id_not_exist(
        &locale,
        Error::Other(format!("x-client-id: {} not exists", x_client_id)),
      ));
    }
  }
}
