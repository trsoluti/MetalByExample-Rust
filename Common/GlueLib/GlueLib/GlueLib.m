//
//  GlueLib.m
//  GlueLib
//
//  Created by TR Solutions on 22/3/20.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

#import "GlueLib.h"

void dispatch_source_set_event_handler_f_with_user_data
(dispatch_source_t _Nonnull source,
 dispatch_function_with_user_data_t _Nullable handler,
 void * _Nullable user_data
 ) {
  //+ NSLog(@"In dispatch source set event handler f with user data, data = %p", user_data);
  dispatch_source_set_event_handler(source, ^() {
    handler(source, user_data);
  });
 }

dispatch_queue_main_t _Nonnull dispatch_get_main_queue_not_inline() {
  return dispatch_get_main_queue();
}
