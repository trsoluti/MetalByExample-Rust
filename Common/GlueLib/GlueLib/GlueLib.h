//
//  GlueLib.h
//  GlueLib
//
//  Created by TR Solutions on 22/3/20.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

#import <Foundation/Foundation.h>
#import <QuartzCore/QuartzCore.h>

/*!
 * @typedef dispatch_function_with_user_data_t
 *
 *  @abstract
 *   The signature of the callback which DisplayLink will call
 *   each time the main display requests a sync.
 *   The callback is executed in the main thread.
 */
typedef void (*dispatch_function_with_user_data_t)(dispatch_source_t _Nonnull , void *_Nullable);

/*!
 * @function dispatch_source_set_event_handler_f_with_user_data
 *
 * @abstract
 * This function allows the caller to pass in arbitrary user data which is passed back when the
 * dispatch source triggers. The data can, for example, allow the callback to be connected to
 * the calling object's id.
 *
 * This function is a kludge  to allow Rust classes to set up dispatch callbacks.
 */
void dispatch_source_set_event_handler_f_with_user_data
(dispatch_source_t _Nonnull,
 dispatch_function_with_user_data_t _Nullable,
  void * _Nullable
  );

/// @function dispatch_get_main_queue_not_inline
///
/// @abstract
/// This is a non-macro version of the Cocoa macro `dispatch_get_main_queue`.
dispatch_queue_main_t _Nonnull dispatch_get_main_queue_not_inline(void);
