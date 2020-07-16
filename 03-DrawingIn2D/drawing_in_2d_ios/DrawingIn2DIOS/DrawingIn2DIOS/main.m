//
//  main.m
//  DrawingIn2DIOS
//
//  Created by TR Solutions on 13/7/20.
//  Copyright © 2020 TR Solutions. All rights reserved.
//

#import <UIKit/UIKit.h>
// #import "AppDelegate.h"

void register_rust_classes(void);

int main(int argc, char * argv[]) {
  register_rust_classes();
//  NSString * appDelegateClassName;
//  @autoreleasepool {
//      // Setup code that might create autoreleased objects goes here.
//      appDelegateClassName = NSStringFromClass([AppDelegate class]);
//  }
  NSString *appDelegateClassName = @"AppDelegate";
  return UIApplicationMain(argc, argv, nil, appDelegateClassName);
}
