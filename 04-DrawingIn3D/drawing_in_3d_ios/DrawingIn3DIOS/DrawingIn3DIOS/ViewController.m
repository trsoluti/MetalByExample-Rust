//
//  ViewController.m
//  DrawingIn3DIOS
//
//  Created by TR Solutions on 18/7/20.
//  Copyright © 2020 TR Solutions Pte. Ltd.
//  Licensed under Apache 2.0 and MIT
//  See appropriate LICENCE files for details.
//

#import "ViewController.h"
#import "MBERenderer.h"
#import "MetalView.h"

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

- (BOOL)prefersStatusBarHidden {
  return YES;
}

@end
