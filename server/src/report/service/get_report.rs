use crate::{
    alias::Pool,
    error::{Error, ServiceError},
    global::Config,
    middleware::Locale,
    report::model::{DbReport, FullReport, Report, ReportFilter, ReportState, ReportType},
    types::{DataWithPageInfo, Image, ImageVersion, ImagesJson, PageInfo, ServiceResult},
};

use sqlx::query_as;
pub async fn get_full_reports(
    locale: &Locale,
    pool: &Pool,
    filter: &ReportFilter,
) -> ServiceResult<DataWithPageInfo<FullReport>> {
    let cfg = Config::global();

    let mut limit = cfg.page_size;
    if let Some(filter_limit) = filter.limit {
        if filter_limit > cfg.max_page_size {
            return Err(ServiceError::bad_request(
                locale,
                "limit_is_too_large",
                Error::Other(format!(
                    "limit {} is too large to max limit {}",
                    filter_limit, cfg.max_page_size
                )),
            ));
        } else {
            limit = filter_limit;
        }
    }
    let rows = query_as!(DbReport,
      r#"
        select id,_type as "_type:ReportType",content,account_id,updated_at,created_at,images,related_post_id,related_account_id,state as "state:ReportState",replied_by,replied_content,replied_at from reports where  ($2::bigint is null or id < $2) and ($3::bigint is null or id > $3) and ($4::bigint is null or state = $4) and ($5::bigint is null or _type = $5)  order by id desc limit $1
  "#,
  limit ,
  filter.after,
  filter.before,
  filter.state as _,
  filter._type as _
    )
    .fetch_all(pool)
    .await?;
    let data: Vec<FullReport> = rows.into_iter().map(|row| format_report(row)).collect();
    let mut start = None;
    let mut end = None;
    if let Some(row) = data.first() {
        start = Some(row.id);
    }
    if let Some(row) = data.last() {
        end = Some(row.id);
    }
    let post_collection = DataWithPageInfo::<FullReport> {
        data,
        page_info: PageInfo { start, end },
    };
    return Ok(post_collection);
}
pub async fn get_reports(
    locale: &Locale,
    pool: &Pool,
    filter: &ReportFilter,
) -> ServiceResult<DataWithPageInfo<Report>> {
    let full_post = get_full_reports(locale, pool, filter).await?;

    let posts = full_post
        .data
        .into_iter()
        .map(|row| Report::from(row))
        .collect();
    return Ok(DataWithPageInfo::<Report> {
        data: posts,
        page_info: full_post.page_info,
    });
}

pub async fn get_full_report(locale: &Locale, pool: &Pool, id: i64) -> ServiceResult<FullReport> {
    let row = query_as!(DbReport,
    r#"
      select id,_type as "_type:ReportType",content,account_id,updated_at,created_at,images,related_post_id,related_account_id,state as "state:ReportState",replied_by,replied_content,replied_at from reports where id=$1
"#,
id
  )
  .fetch_optional(pool)
  .await?;
    if let Some(row) = row {
        return Ok(format_report(row));
    } else {
        return Err(ServiceError::record_not_exist(
            locale,
            "report_not_exists",
            Error::Other(format!("Can not found post template id: {} at db", id)),
        ));
    }
}

pub async fn get_report(locale: &Locale, pool: &Pool, id: i64) -> ServiceResult<Report> {
    return Ok(get_full_report(locale, pool, id).await?.into());
}

pub fn format_report(row: DbReport) -> FullReport {
    let DbReport {
        id,
        _type,
        content,
        account_id,
        updated_at,
        created_at,
        images: db_images_value,
        related_post_id,
        related_account_id,
        state,
        replied_by,
        replied_content,
        replied_at,
    } = row;

    let mut images: Vec<Image> = Vec::new();
    if let Some(images_value) = db_images_value {
        let db_images: ImagesJson = serde_json::from_value(images_value).unwrap_or(ImagesJson {
            version: ImageVersion::V1,
            images: Vec::new(),
        });
        images = db_images.images;
    }
    return FullReport {
        id,
        _type,
        content,
        account_id,
        updated_at,
        created_at,
        images,
        related_post_id,
        related_account_id,
        state,
        replied_by,
        replied_content,
        replied_at,
    };
}
