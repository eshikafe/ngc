// Copyright (c) 2022 VPlane Telecoms

#![allow(dead_code)]
// #include "context.h"
// #include "fd-path.h"
// #include "gtp-path.h"
// #include "pfcp-path.h"
// #include "sbi-path.h"

// static ogs_thread_t *thread;
use std::thread;

// static void smf_main(void *data);

// static int initialized = 0;
// static ogs_timer_t *t_termination_holding = NULL;

pub fn smf_initialize() -> Result<(), &'static str> {
    // int rv;

    // ogs_gtp_context_init(ogs_app()->pool.nf * OGS_MAX_NUM_OF_GTPU_RESOURCE);
    // ogs_pfcp_context_init();

    // smf_context_init();
    // smf_event_init();
    // ogs_sbi_context_init();

    // rv = ogs_gtp_xact_init();
    // if (rv != OGS_OK) return rv;

    // rv = ogs_pfcp_xact_init();
    // if (rv != OGS_OK) return rv;

    // rv = ogs_gtp_context_parse_config("smf", "upf");
    // if (rv != OGS_OK) return rv;

    // rv = ogs_pfcp_context_parse_config("smf", "upf");
    // if (rv != OGS_OK) return rv;

    // rv = ogs_sbi_context_parse_config("smf", "nrf");
    // if (rv != OGS_OK) return rv;

    // rv = smf_context_parse_config();
    // if (rv != OGS_OK) return rv;

    // rv = ogs_log_config_domain(
    //         ogs_app()->logger.domain, ogs_app()->logger.level);
    // if (rv != OGS_OK) return rv;

    // rv = ogs_pfcp_ue_pool_generate();
    // if (rv != OGS_OK) return rv;

    // rv = smf_fd_init();
    // if (rv != 0) return OGS_ERROR;

    // rv = smf_gtp_open();
    // if (rv != 0) return OGS_ERROR;

    // rv = smf_pfcp_open();
    // if (rv != 0) return OGS_ERROR;

    // rv = smf_sbi_open();
    // if (rv != 0) return OGS_ERROR;

    // thread = ogs_thread_create(smf_main, NULL);
    thread::spawn(smf_main); // tokio::spawn(async move {smf_main().await})
    // if (!thread) return OGS_ERROR;

    // initialized = 1;

    // return OGS_OK;
    Ok(())
}

fn event_termination() {
//     ogs_sbi_nf_instance_t *nf_instance = NULL;

//     /* Sending NF Instance De-registeration to NRF */
//     ogs_list_for_each(&ogs_sbi_self()->nf_instance_list, nf_instance)
//         smf_nf_fsm_fini(nf_instance);

//     /* Starting holding timer */
//     t_termination_holding = ogs_timer_add(ogs_app()->timer_mgr, NULL, NULL);
//     ogs_assert(t_termination_holding);
// #define TERMINATION_HOLDING_TIME ogs_time_from_msec(300)
//     ogs_timer_start(t_termination_holding, TERMINATION_HOLDING_TIME);

//     /* Sending termination event to the queue */
//     ogs_queue_term(ogs_app()->queue);
//     ogs_pollset_notify(ogs_app()->pollset);
}

pub fn smf_terminate(){
//     if (!initialized) return;

//     /* Daemon terminating */
//     event_termination();
//     ogs_thread_destroy(thread);
//     ogs_timer_delete(t_termination_holding);

//     smf_gtp_close();
//     smf_pfcp_close();
//     smf_sbi_close();

//     smf_fd_final();

//     smf_context_final();

//     ogs_pfcp_context_final();
//     ogs_sbi_context_final();
//     ogs_gtp_context_final();

//     ogs_pfcp_xact_final();
//     ogs_gtp_xact_final();

//     smf_event_final(); /* Destroy event */
}

async fn smf_main(){
//     ogs_fsm_t smf_sm;
//     int rv;

//     ogs_fsm_create(&smf_sm, smf_state_initial, smf_state_final);
//     ogs_fsm_init(&smf_sm, 0);

//     for ( ;; ) {
//         ogs_pollset_poll(ogs_app()->pollset,
//                 ogs_timer_mgr_next(ogs_app()->timer_mgr));

//         /*
//          * After ogs_pollset_poll(), ogs_timer_mgr_expire() must be called.
//          *
//          * The reason is why ogs_timer_mgr_next() can get the corrent value
//          * when ogs_timer_stop() is called internally in ogs_timer_mgr_expire().
//          *
//          * You should not use event-queue before ogs_timer_mgr_expire().
//          * In this case, ogs_timer_mgr_expire() does not work
//          * because 'if rv == OGS_DONE' statement is exiting and
//          * not calling ogs_timer_mgr_expire().
//          */
//         ogs_timer_mgr_expire(ogs_app()->timer_mgr);

//         for ( ;; ) {
//             smf_event_t *e = NULL;

//             rv = ogs_queue_trypop(ogs_app()->queue, (void**)&e);
//             ogs_assert(rv != OGS_ERROR);

//             if (rv == OGS_DONE)
//                 goto done;

//             if (rv == OGS_RETRY)
//                 break;

//             ogs_assert(e);
//             ogs_fsm_dispatch(&smf_sm, e);
//             smf_event_free(e);
//         }
//     }
// done:

//     ogs_fsm_fini(&smf_sm, 0);
//     ogs_fsm_delete(&smf_sm);
}