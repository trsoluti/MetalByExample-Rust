//
//  CAMDisplayLink.m
//  DrawingIn2DMacOS
//
//  Created by TR Solutions on 14/7/20.
//  Copyright © 2020 TR Solutions. All rights reserved.
//

#import "CAVDisplayLink.h"
@import CoreVideo;
#import "GlueLib.h"
@import ObjectiveC;

@interface CAVDisplayLink ()
@property CVDisplayLinkRef displayLinkRef;
@property (nonatomic, strong) dispatch_source_t source;
@property (nonatomic, strong) id target;
@property SEL selector;
@property (readwrite, nonatomic) NSTimeInterval duration;
@end

@implementation CAVDisplayLink

+ (CAVDisplayLink *)displayLinkWithTarget:(id)target
                                 selector:(SEL)sel
                         didFailWithError: (NSError * _Nullable __autoreleasing *)error
{
  CAVDisplayLink * displayLink = [[CAVDisplayLink alloc] initWithTarget:(id)target
                                                               selector:(SEL)sel
                                                       didFailWithError:(NSError * _Nullable __autoreleasing *)error];
  return displayLink;
}

- (instancetype) initWithTarget:(id)target
                       selector:(SEL)sel
               didFailWithError: (NSError * _Nullable __autoreleasing *)error {
  //+ NSLog(@"In CAVDisplayLink initWithTarget");
  if (self = [super init]) {
    _target = target;
    _selector = sel;
    _source = dispatch_source_create(DISPATCH_SOURCE_TYPE_DATA_ADD, 0, 0, dispatch_get_main_queue());
    
    // Get a display link pointer
    CVDisplayLinkRef displayLinkRef;
    CGDirectDisplayID mainDisplay = CGMainDisplayID();
    CVDisplayLinkCreateWithCGDisplay(mainDisplay, &displayLinkRef);
    if (displayLinkRef == nil) {
      if (error != nil) {
        NSString *domain = @"com.trsolutions.DisplayLink.InitializationErrors";
           NSString *desc = NSLocalizedString(@"Unable to create display link", @"");
           NSDictionary *userInfo = @{ NSLocalizedDescriptionKey : desc };
        
          *error = [NSError errorWithDomain:domain
                                       code: FAILED_TO_CREATE_DISPLAY_LINK
                                   userInfo:userInfo];

      }
      return nil;
    }
    _displayLinkRef = CVDisplayLinkRetain(displayLinkRef);
    
    CVReturn returnCode = CVDisplayLinkSetOutputCallback(displayLinkRef, displayLinkCallback, (__bridge void * _Nullable)(_source));
    
    if (returnCode != kCVReturnSuccess) {
      if (error != nil) {
        NSString *domain = @"com.trsolutions.DisplayLink.InitializationErrors";
           NSString *desc = NSLocalizedString(@"Unable to link output handler", @"");
           NSDictionary *userInfo = @{ NSLocalizedDescriptionKey : desc };
        
          *error = [NSError errorWithDomain:domain
                                       code:FAILED_TO_LINK_OUTPUT_HANDLER
                                   userInfo:userInfo];
      }
      return nil;
    }
    
    dispatch_source_set_event_handler_f_with_user_data(_source, &dispatch_event_handler_f, (__bridge void * _Nullable)(self));
    
    if (error != nil) {
      *error = nil;
    }
    NSLog(@"Display Link successfully initialized.");
  }
  return self;
}

- (void)addToRunLoop:(NSRunLoop *)runloop forMode:(NSRunLoopMode)mode {
  CVDisplayLinkStart(_displayLinkRef);
  dispatch_resume(_source);
}


- (void)invalidate {
  CVDisplayLinkStop(_displayLinkRef);
  dispatch_suspend(_source);
}

#pragma mark - Callback

static CVReturn displayLinkCallback
 (
    CVDisplayLinkRef displayLink,
    const CVTimeStamp* now,
    const CVTimeStamp* outputTime,
    CVOptionFlags flagsIn,
    CVOptionFlags* flagsOut,
    void* displayLinkContext
    )
{
  //+ NSLog(@"in displayLinkCallback");
  NSObject<OS_dispatch_source> * source = (__bridge NSObject<OS_dispatch_source> *)displayLinkContext;
  //+ NSLog(@"displayLinkContext = %@", displayLinkContext);
  dispatch_source_merge_data(source, 1);
  return kCVReturnSuccess;
}

static void dispatch_event_handler_f(dispatch_source_t _Nonnull source, void* _Nullable user_data) {
  //+ NSLog(@"In dispatch event handler f, user_data = %@", user_data);
  CAVDisplayLink *displayLink = (__bridge CAVDisplayLink *) user_data;
  if (displayLink != nil && displayLink.target != nil) {
    displayLink.duration = 1. / 60.;
    IMP imp = [displayLink.target methodForSelector:displayLink.selector];
    void (*func)(id, SEL) = (void*) imp;
    func(displayLink.target, displayLink.selector);
  }
}


@end
