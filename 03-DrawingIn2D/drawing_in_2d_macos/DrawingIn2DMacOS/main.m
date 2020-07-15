//
//  main.m
//  DrawingIn2DMacOS
//
//  Created by TR Solutions on 14/7/20.
//  Copyright © 2020 TR Solutions. All rights reserved.
//

#import <Cocoa/Cocoa.h>

void register_rust_classes(void);

int main(int argc, const char * argv[]) {
  register_rust_classes();
  @autoreleasepool {
      // Setup code that might create autoreleased objects goes here.
  }
  return NSApplicationMain(argc, argv);
}
