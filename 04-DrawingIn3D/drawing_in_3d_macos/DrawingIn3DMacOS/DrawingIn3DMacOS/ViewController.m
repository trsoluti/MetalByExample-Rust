//
//  ViewController.m
//  DrawingIn3DMacOS
//
//  Created by TR Solutions on 23/7/20.
//  Copyright © 2020 TR Solutions. All rights reserved.
//

#import "ViewController.h"
#import "MBEItems/MBERenderer.h"
#import "MBEItems/MetalView.h"

@interface ViewController ()
@property (nonatomic, strong) MBERenderer *renderer;
@end


@implementation ViewController

- (MetalView *)metalView {
  return (MetalView *)self.view;
}

- (void)viewDidLoad {
  [super viewDidLoad];

  // Do any additional setup after loading the view.
  self.renderer = [MBERenderer new];
  self.metalView.delegate = self.renderer;
}


- (void)setRepresentedObject:(id)representedObject {
  [super setRepresentedObject:representedObject];

  // Update the view, if already loaded.
}


@end
