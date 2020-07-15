//
//  MetalView.h
//  DrawingIn2DMacOS
//
//  Created by TR Solutions on 14/7/20.
//  Copyright © 2020 TR Solutions. All rights reserved.
//

#import <Cocoa/Cocoa.h>

NS_ASSUME_NONNULL_BEGIN

/// @interface MetalView
/// @abstract A class to handle the interface to Metal while operating as an NSView.
/// @discussion
/// This class provides an interface similar to MetalKit's MTKView. It syncs with the display
@interface MetalView: NSView

@end

NS_ASSUME_NONNULL_END
