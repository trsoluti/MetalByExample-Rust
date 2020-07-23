//
//  CAMDisplayLink.h
//  DrawingIn2DMacOS
//
//  Created by TR Solutions on 14/7/20.
//  Copyright © 2020 TR Solutions. All rights reserved.
//

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/// @abstract Error codes returned by the the display link displayLinkWithTarget: selector:didFailWithError: method
/// @constant FAILED_TO_CREATE_DISPLAY_LINK Attempt to connect to the timer failed
/// @constant FAILED_TO_CONNECT_DISPLAY Attempt to connect to the main display failed
/// @discussion only valid if error is not nil on return from displayLinkWithTarget: selector:didFailWithError:
enum DisplayLinkError {
  FAILED_TO_CREATE_DISPLAY_LINK = -101,
  FAILED_TO_LINK_OUTPUT_HANDLER = -102,
  FAILED_TO_CONNECT_DISPLAY = -103
};


@interface CAVDisplayLink : NSObject

@property (nonatomic) NSInteger preferredFramesPerSecond;
@property(readonly, nonatomic) CFTimeInterval duration;

+ (CAVDisplayLink *)displayLinkWithTarget:(id)target
                                 selector:(SEL)sel
                         didFailWithError: (NSError * _Nullable __autoreleasing *)error;

- (void)addToRunLoop:(NSRunLoop *)runloop forMode:(NSRunLoopMode)mode;

- (void)invalidate;

@end

NS_ASSUME_NONNULL_END
