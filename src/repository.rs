// fn get_epic(pool: &Pool, epic_id: u64) -> Result<Epic> {
//     let mut conn = pool.get_conn()?;

//     let rows = conn.exec_map(
//         r"SELECT
//             e.id, e.name as epic_name, e.description as epic_description, e.status as epic_status,
//             s.name as story_name, s.description as story_description, s.status as story_status
//           FROM epics e
//           LEFT JOIN stories s ON e.id = s.epic_id
//           WHERE e.id = :epic_id",
//         params! {
//             "epic_id" => epic_id
//         },
//         |(e_id, e_name, e_desc, e_status, s_id, s_name, s_desc, s_status)| {
//             (
//                 e_id, e_name, e_desc, e_status, s_id, s_name, s_desc, s_status,
//             )
//         },
//     )?;
// }
