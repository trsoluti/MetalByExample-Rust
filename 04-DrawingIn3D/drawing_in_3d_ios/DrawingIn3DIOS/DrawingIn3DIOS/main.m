//
//  main.m
//  DrawingIn3DIOS
//
//  Created by TR Solutions on 18/7/20.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

#import <UIKit/UIKit.h>
#import "AppDelegate.h"

void register_rust_classes(void);

int main(int argc, char * argv[]) {
  register_rust_classes();
//  NSString * appDelegateClassName;
//  @autoreleasepool {
//      // Setup code that might create autoreleased objects goes here.
//      appDelegateClassName = NSStringFromClass([AppDelegate class]);
//  }
//  return UIApplicationMain(argc, argv, nil, appDelegateClassName);
  return UIApplicationMain(argc, argv, nil, @"AppDelegate");
}
