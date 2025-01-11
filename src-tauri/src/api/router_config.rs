use crate::api::{cloud_import, music_first_listen_info, send_album, voicelist_list_search, voicelist_search, voicelist_trans, weblog, yunbei};
use crate::api::{yunbei_expense, yunbei_info, yunbei_rcmd_song, yunbei_rcmd_song_history};
use crate::api::{yunbei_receipt, yunbei_sign, yunbei_task_finish, yunbei_tasks, yunbei_tasks_todo};
use crate::api::{video_timeline_recommend, video_url, vip_growthpoint, vip_growthpoint_details};
use crate::api::{vip_growthpoint_get, vip_info, vip_info_v2, vip_tasks, vip_timemachine};
use crate::api::{voice_delete, voice_detail, voice_lyric, voicelist_detail, voicelist_list};
use crate::api::{style_song, summary_annual, threshold_detail_get, top_album, top_artists};
use crate::api::{top_list, top_mv, top_playlist, top_playlist_highquality, top_song, };
use crate::api::{topic_detail, topic_detail_event_hot, topic_sublist, toplist, toplist_artist};
use crate::api::{toplist_detail, ugc_album_get, ugc_artist_get, ugc_artist_search, ugc_detail};
use crate::api::{ugc_mv_get, ugc_song_get, ugc_user_devote, user_account, user_audio, user_binding};
use crate::api::{user_bindingcellphone, user_cloud, user_cloud_del, user_cloud_detail, };
use crate::api::{user_comment_history, user_detail, user_dj, user_event, user_follow_mixed};
use crate::api::{user_followeds, user_follows, user_level, user_medal, user_mutualfollow_get};
use crate::api::{user_playlist, user_record, user_replacephone, user_social_status};
use crate::api::{user_social_status_edit, user_social_status_rcmd, user_social_status_support};
use crate::api::{user_subcount, user_update, verify_get_qr, video_category_list, video_detail};
use crate::api::{style_artist, style_detail, style_list, style_playlist, style_preference};
use crate::api::{song_url_v1, song_wiki_summary, starpick_comments_summary, style_album};
use crate::api::{song_order_update, song_purchased, song_red_count, song_singledownlist, song_url};
use crate::api::{song_dynamic_cover, song_like_check, song_monthdownlist, song_music_detail};
use crate::api::{simi_user, song_detail, song_downlist, song_download_url, song_download_url_v1};
use crate::api::{sign_happy_info, signin_progress, simi_artist, simi_mv, simi_playlist, simi_song};
use crate::api::{send_song, send_text, setting, share_resource, sheet_list, sheet_preview};
use crate::api::{search_hot_detail, search_match, search_multimatch, search_suggest, send_playlist};
use crate::api::{related_playlist, resource_like, scrobble, search, search_default, search_hot};
use crate::api::{record_recent_video, record_recent_voice, register_cellphone, related_allvideo};
use crate::api::{record_recent_album, record_recent_dj, record_recent_playlist, record_recent_song};
use crate::api::{recent_listen_list, recommend_resource, recommend_songs, recommend_songs_dislike};
use crate::api::{playmode_intelligence_list, playmode_song_vector, program_recommend, rebind};
use crate::api::{playlist_tracks, playlist_update, playlist_update_playcount, };
use crate::api::{playlist_tags_update, playlist_track_add, playlist_track_all, };
use crate::api::{playlist_track_delete,playlist_video_recent,video_detail_info };
use crate::api::{video_group,video_group_list,video_sub,video_timeline_all};
use crate::api::{playlist_order_update, playlist_privacy, playlist_subscribe, playlist_subscribers};
use crate::api::{playlist_import_task_status, playlist_mylike, playlist_name_update,dj_paygift};
use crate::api::{playlist_highquality_tags, playlist_hot, playlist_import_name_task_create};
use crate::api::{playlist_delete, playlist_desc_update, playlist_detail, playlist_detail_dynamic};
use crate::api::{personalized_privatecontent_list, pl_count, playlist_catlist, playlist_create};
use crate::api::{personalized_mv, personalized_newsong, personalized_privatecontent};
use crate::api::{personal_fm, personal_fm_mode, personalized, personalized_djprogram};
use crate::api::{cellphone_existence_check, cloud_match, mv_sublist, mv_url, nickname_check};
use crate::api::{fanscenter_basicinfo_province_get, fanscenter_overview_get, fanscenter_trend_list};
use crate::api::{listen_data_today_song, listen_data_total, listen_data_year_report};
use crate::api::{listentogether_accept, listentogether_end, listentogether_heatbeat};
use crate::api::{listentogether_play_command, listentogether_room_check, };
use crate::api::{yunbei_today,listentogether_sync_playlist_get,listentogether_room_create};
use crate::api::{listentogether_status, listentogether_sync_list_command, };
use crate::api::{album_detail_dynamic,login_refresh,dj_recommend_type,artist_sublist};
use crate::api::{login, login_cellphone, login_qr_check, login_qr_create, login_qr_key, };
use crate::api::{homepage_dragon_ball, likelist, listen_data_realtime_report, listen_data_report,};
use crate::api::{logout, lyric, lyric_new, mlog_music_rcmd, mlog_to_video, mlog_url, msg_comments};
use crate::api::{msg_forwards, msg_notices, login_status, mv_exclusive_rcmd, mv_first, mv_sub};
use crate::api::{musician_cloudbean_obtain, musician_data_overview, musician_play_trend};
use crate::api::{msg_private, msg_private_history, msg_recentcontact, musician_cloudbean};
use crate::api::{history_recommend_songs, history_recommend_songs_detail, homepage_block_page};
use crate::api::{musician_sign, musician_tasks, musician_tasks_new, mv_all, mv_detail, };
use crate::api::{fanscenter_basicinfo_age_get, fanscenter_basicinfo_gender_get};
use crate::api::{dj_today_perfered, dj_toplist_hours, dj_toplist_newcomer, dj_toplist_popular};
use crate::api::{dj_personalize_recommend, dj_program_detail, dj_program_toplist, };
use crate::api::{digitalAlbum_sales, dj_category_excludehot, dj_category_recommend};
use crate::api::{digitalAlbum_detail, digitalAlbum_ordering, digitalAlbum_purchased};
use crate::api::{countries_code_list, creator_authinfo_get, hot_topic, hug_comment, like};
use crate::api::{artist_detail_dynamic, artist_follow_count, comment_hug_list, comment_playlist};
use crate::api::{eapi_decrypt, event, event_del, event_forward, fm_trash, follow, get_userids};
use crate::api::{album_list, album_list_style, album_new, artist_fans, artist_list, djRadio_top};
use crate::api::{artist_album, artist_desc, artist_detail, check_music, captcha_sent};
use crate::api::{activate_init_profile, aijd_content_rcmd, album, album_detail, };
use crate::api::{album_newest, album_privilege, album_songsaleboard, album_sub, album_sublist, api};
use crate::api::{artist_mv, artist_new_mv, artist_new_song, artist_songs, artist_sub, };
use crate::api::{artist_top_song, artist_video, artists, audio_match, banner, batch, calendar};
use crate::api::{captcha_verify, cloudsearch, comment, comment_album, comment_dj, comment_event};
use crate::api::{comment_floor, comment_hot, comment_like, comment_music, comment_mv, comment_new};
use crate::api::{comment_video, daily_signin, dj_banner, dj_catelist, dj_detail, dj_hot};
use crate::api::{dj_program, dj_radio_hot, dj_recommend, dj_sub, dj_sublist, dj_subscriber};
use crate::api::{dj_toplist, dj_toplist_pay,dj_program_toplist_hours,mv_detail_info};

use actix_web::web;

// 路由配置函数，集中管理不同模块的路由配置
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .configure(check_music::configure)
        .configure(activate_init_profile::configure)
        .configure(aijd_content_rcmd::configure)
        .configure(album::configure)
        .configure(album_detail::configure)
        .configure(album_detail_dynamic::configure)
        .configure(album_list::configure)
        .configure(album_list_style::configure)
        .configure(album_new::configure)
        .configure(album_newest::configure)
        .configure(album_privilege::configure)
        .configure(album_songsaleboard::configure)
        .configure(album_sub::configure)
        .configure(album_sublist::configure)
        .configure(api::configure)
        .configure(artist_album::configure)
        .configure(artist_desc::configure)
        .configure(artist_detail::configure)
        .configure(artist_fans::configure)
        .configure(artist_list::configure)
        .configure(artist_mv::configure)
        .configure(artist_new_mv::configure)
        .configure(artist_new_song::configure)
        .configure(artist_songs::configure)
        .configure(artist_sub::configure)
        .configure(artist_sublist::configure)
        .configure(artist_top_song::configure)
        .configure(artist_video::configure)
        .configure(artists::configure)
        .configure(audio_match::configure)
        .configure(artist_detail_dynamic::configure)
        .configure(artist_follow_count::configure)

        .configure(banner::configure)
        .configure(batch::configure)


        .configure(calendar::configure)
        .configure(captcha_sent::configure)
        .configure(captcha_verify::configure)
        .configure(cloudsearch::configure)
        .configure(comment::configure)
        .configure(comment_album::configure)
        .configure(comment_dj::configure)
        .configure(comment_event::configure)
        .configure(comment_floor::configure)
        .configure(comment_hot::configure)
        .configure(comment_like::configure)
        .configure(comment_music::configure)
        .configure(comment_mv::configure)
        .configure(comment_new::configure)
        .configure(comment_video::configure)
        .configure(comment_hug_list::configure)
        .configure(comment_playlist::configure)
        .configure(countries_code_list::configure)
        .configure(creator_authinfo_get::configure)
        .configure(cellphone_existence_check::configure)
        .configure(cloud_match::configure)
        .configure(cloud_import::configure)


        .configure(daily_signin::configure)
        .configure(dj_banner::configure)
        .configure(dj_catelist::configure)
        .configure(dj_detail::configure)
        .configure(dj_hot::configure)
        .configure(dj_paygift::configure)
        .configure(dj_program::configure)
        .configure(dj_radio_hot::configure)
        .configure(dj_recommend::configure)
        .configure(dj_sub::configure)
        .configure(dj_sublist::configure)
        .configure(dj_subscriber::configure)
        .configure(dj_toplist::configure)
        .configure(dj_toplist_pay::configure)
        .configure(djRadio_top::configure)
        .configure(digitalAlbum_detail::configure)
        .configure(digitalAlbum_ordering::configure)
        .configure(digitalAlbum_purchased::configure)
        .configure(digitalAlbum_sales::configure)
        .configure(dj_category_excludehot::configure)
        .configure(dj_category_recommend::configure)
        .configure(dj_personalize_recommend::configure)
        .configure(dj_program_detail::configure)
        .configure(dj_program_toplist::configure)
        .configure(dj_program_toplist_hours::configure)
        .configure(dj_recommend_type::configure)
        .configure(dj_today_perfered::configure)
        .configure(dj_toplist_hours::configure)
        .configure(dj_toplist_newcomer::configure)
        .configure(dj_toplist_popular::configure)

        .configure(eapi_decrypt::configure)
        .configure(event::configure)
        .configure(event_del::configure)
        .configure(event_forward::configure)



        .configure(fanscenter_basicinfo_age_get::configure)
        .configure(fanscenter_basicinfo_gender_get::configure)
        .configure(fanscenter_basicinfo_province_get::configure)
        .configure(fanscenter_overview_get::configure)
        .configure(fanscenter_trend_list::configure)


        .configure(fm_trash::configure)
        .configure(follow::configure)

        .configure(get_userids::configure)

        .configure(hot_topic::configure)
        .configure(hug_comment::configure)
        .configure(history_recommend_songs::configure)
        .configure(history_recommend_songs_detail::configure)
        .configure(homepage_block_page::configure)
        .configure(homepage_dragon_ball::configure)


        .configure(like::configure)
        .configure(likelist::configure)
        .configure(listen_data_realtime_report::configure)
        .configure(listen_data_report::configure)
        .configure(listen_data_today_song::configure)
        .configure(listen_data_total::configure)
        .configure(listen_data_year_report::configure)
        .configure(listentogether_accept::configure)
        .configure(listentogether_end::configure)
        .configure(listentogether_heatbeat::configure)
        .configure(listentogether_play_command::configure)
        .configure(listentogether_room_check::configure)
        .configure(listentogether_room_create::configure)
        .configure(listentogether_status::configure)
        .configure(listentogether_sync_list_command::configure)
        .configure(listentogether_sync_playlist_get::configure)
        .configure(login::configure)
        .configure(login_cellphone::configure)
        .configure(login_qr_check::configure)
        .configure(login_qr_create::configure)
        .configure(login_qr_key::configure)
        .configure(login_refresh::configure)
        .configure(login_status::configure)
        .configure(logout::configure)
        .configure(lyric::configure)
        .configure(lyric_new::configure)



        .configure(mlog_music_rcmd::configure)
        .configure(mlog_url::configure)
        .configure(mlog_to_video::configure)

        .configure(msg_comments::configure)
        .configure(msg_forwards::configure)
        .configure(msg_notices::configure)
        .configure(msg_private::configure)
        .configure(msg_private_history::configure)
        .configure(msg_recentcontact::configure)
        .configure(musician_cloudbean::configure)
        .configure(musician_cloudbean_obtain::configure)
        .configure(musician_data_overview::configure)
        .configure(musician_play_trend::configure)
        .configure(musician_sign::configure)
        .configure(musician_tasks_new::configure)
        .configure(musician_tasks::configure)
        .configure(mv_all::configure)
        .configure(mv_detail::configure)
        .configure(mv_detail_info::configure)
        .configure(mv_exclusive_rcmd::configure)
        .configure(mv_first::configure)
        .configure(mv_sub::configure)
        .configure(mv_sublist::configure)
        .configure(mv_url::configure)
        .configure(music_first_listen_info::configure)

        .configure(nickname_check::configure)

        .configure(personal_fm::configure)
        .configure(personal_fm_mode::configure)
        .configure(personalized::configure)
        .configure(personalized_djprogram::configure)
        .configure(personalized_mv::configure)
        .configure(personalized_newsong::configure)
        .configure(personalized_privatecontent_list::configure)
        .configure(pl_count::configure)
        .configure(playlist_catlist::configure)
        .configure(playlist_create::configure)
        .configure(playlist_delete::configure)
        .configure(playlist_desc_update::configure)
        .configure(playlist_detail::configure)
        .configure(playlist_detail_dynamic::configure)
        .configure(playlist_highquality_tags::configure)
        .configure(playlist_hot::configure)
        .configure(playlist_import_name_task_create::configure)
        .configure(playlist_import_task_status::configure)
        .configure(playlist_mylike::configure)
        .configure(playlist_name_update::configure)
        .configure(playlist_order_update::configure)
        .configure(playlist_privacy::configure)
        .configure(playlist_subscribe::configure)
        .configure(playlist_subscribers::configure)
        .configure(playlist_tags_update::configure)
        .configure(playlist_track_add::configure)
        .configure(playlist_track_all::configure)
        .configure(playlist_track_delete::configure)
        .configure(playlist_update::configure)
        .configure(playlist_update_playcount::configure)
        .configure(playlist_video_recent::configure)
        .configure(playmode_intelligence_list::configure)
        .configure(playmode_song_vector::configure)
        .configure(program_recommend::configure)
        .configure(personalized_privatecontent::configure)
        .configure(playlist_tracks::configure)
        
        .configure(rebind::configure)
        .configure(recent_listen_list::configure)
        .configure(recommend_resource::configure)
        .configure(recommend_songs::configure)
        .configure(recommend_songs_dislike::configure)
        .configure(record_recent_album::configure)
        .configure(record_recent_dj::configure)
        .configure(record_recent_playlist::configure)
        .configure(record_recent_song::configure)
        .configure(record_recent_video::configure)
        .configure(record_recent_voice::configure)
        .configure(register_cellphone::configure)
        .configure(related_allvideo::configure)
        .configure(related_playlist::configure)
        .configure(resource_like::configure)
        
        .configure(scrobble::configure)
        .configure(send_album::configure)
        .configure(search::configure)
        .configure(search_default::configure)
        .configure(search_hot::configure)
        .configure(search_hot_detail::configure)
        .configure(search_match::configure)
        .configure(search_multimatch::configure)
        .configure(search_suggest::configure)
        .configure(send_playlist::configure)
        .configure(send_song::configure)
        .configure(send_text::configure)
        .configure(setting::configure)
        .configure(share_resource::configure)
        .configure(sheet_list::configure)
        .configure(sheet_preview::configure)
        .configure(sign_happy_info::configure)
        .configure(signin_progress::configure)
        .configure(simi_artist::configure)
        .configure(simi_mv::configure)
        .configure(simi_playlist::configure)
        .configure(simi_song::configure)
        .configure(simi_user::configure)
        .configure(song_detail::configure)
        .configure(song_downlist::configure)
        .configure(song_dynamic_cover::configure)
        .configure(song_like_check::configure)
        .configure(song_monthdownlist::configure)
        .configure(song_monthdownlist::configure)
        .configure(song_music_detail::configure)
        .configure(song_order_update::configure)
        .configure(song_purchased::configure)
        .configure(song_red_count::configure)
        .configure(song_singledownlist::configure)
        .configure(song_url::configure)
        .configure(song_url_v1::configure)
        .configure(song_wiki_summary::configure)
        .configure(starpick_comments_summary::configure)
        .configure(style_album::configure)
        .configure(style_artist::configure)
        .configure(style_detail::configure)
        .configure(style_list::configure)
        .configure(style_playlist::configure)
        .configure(style_preference::configure)
        .configure(style_song::configure)
        .configure(summary_annual::configure)
        .configure(song_download_url::configure)
        .configure(song_download_url_v1::configure)
        
        .configure(threshold_detail_get::configure)
        .configure(top_album::configure)
        .configure(top_artists::configure)
        .configure(top_list::configure)
        .configure(top_mv::configure)
        .configure(top_playlist::configure)
        .configure(top_playlist_highquality::configure)
        .configure(top_song::configure)
        .configure(topic_detail::configure)
        .configure(topic_detail_event_hot::configure)
        .configure(topic_sublist::configure)
        .configure(toplist::configure)
        .configure(toplist_artist::configure)
        .configure(toplist_detail::configure)
        
        
        .configure(ugc_album_get::configure)
        .configure(ugc_artist_get::configure)
        .configure(ugc_artist_search::configure)
        .configure(ugc_detail::configure)
        .configure(ugc_mv_get::configure)
        .configure(ugc_song_get::configure)
        .configure(ugc_user_devote::configure)
        .configure(user_account::configure)
        .configure(user_audio::configure)
        .configure(user_binding::configure)
        .configure(user_bindingcellphone::configure)
        .configure(user_cloud::configure)
        .configure(user_cloud_del::configure)
        .configure(user_cloud_detail::configure)
        .configure(user_comment_history::configure)
        .configure(user_detail::configure)
        .configure(user_dj::configure)
        .configure(user_event::configure)
        .configure(user_follow_mixed::configure)
        .configure(user_followeds::configure)
        .configure(user_follows::configure)
        .configure(user_level::configure)
        .configure(user_medal::configure)
        .configure(user_mutualfollow_get::configure)
        .configure(user_playlist::configure)
        .configure(user_record::configure)
        .configure(user_replacephone::configure)
        .configure(user_social_status::configure)
        .configure(user_social_status_edit::configure)
        .configure(user_social_status_rcmd::configure)
        .configure(user_social_status_support::configure)
        .configure(user_subcount::configure)
        .configure(user_update::configure)
        
        
        .configure(verify_get_qr::configure)
        .configure(video_category_list::configure)
        .configure(video_detail::configure)
        .configure(video_detail_info::configure)
        .configure(video_group::configure)
        .configure(video_group_list::configure)
        .configure(video_sub::configure)
        .configure(video_timeline_all::configure)
        .configure(video_timeline_recommend::configure)
        .configure(video_url::configure)
        .configure(vip_growthpoint::configure)
        .configure(vip_growthpoint_details::configure)
        .configure(vip_growthpoint_get::configure)
        .configure(vip_info::configure)
        .configure(vip_info_v2::configure)
        .configure(vip_tasks::configure)
        .configure(vip_timemachine::configure)
        .configure(voice_delete::configure)
        .configure(voice_detail::configure)
        .configure(voice_lyric::configure)
        .configure(voicelist_detail::configure)
        .configure(voicelist_list::configure)
        .configure(voicelist_list_search::configure)
        .configure(voicelist_search::configure)
        .configure(voicelist_trans::configure)
        
        .configure(weblog::configure)
        
        .configure(yunbei::configure)
        .configure(yunbei_expense::configure)
        .configure(yunbei_info::configure)
        .configure(yunbei_rcmd_song::configure)
        .configure(yunbei_rcmd_song_history::configure)
        .configure(yunbei_receipt::configure)
        .configure(yunbei_sign::configure)
        .configure(yunbei_task_finish::configure)
        .configure(yunbei_tasks::configure)
        .configure(yunbei_tasks_todo::configure)
        .configure(yunbei_today::configure)
    
    ;
}
