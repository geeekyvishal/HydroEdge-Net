// HydroEdge-Net: Bare-Metal Memory-Safe Rust for Embedded No-STD Systems
#![no_std]

pub const NUM_FEATURES: usize = 15;

#[inline(always)]
fn tree_0(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 1.985371_f32 {
        if features[5] <= 25.860913_f32 {
            if features[11] <= 3.433234_f32 {
                if features[14] <= 1.629720_f32 {
                    if features[10] <= 45.822155_f32 {
                        0.759972_f32
                    } else {
                        0.910550_f32
                    }
                } else {
                    0.923740_f32
                }
            } else {
                if features[11] <= 4.518059_f32 {
                    if features[3] <= 28.448701_f32 {
                        0.747270_f32
                    } else {
                        0.650599_f32
                    }
                } else {
                    0.840559_f32
                }
            }
        } else {
            if features[13] <= 6.570718_f32 {
                if features[6] <= 1.833727_f32 {
                    0.856856_f32
                } else {
                    0.976001_f32
                }
            } else {
                if features[2] <= 30.838671_f32 {
                    if features[5] <= 41.697684_f32 {
                        0.914952_f32
                    } else {
                        0.775851_f32
                    }
                } else {
                    0.768214_f32
                }
            }
        }
    } else {
        if features[5] <= 2.558450_f32 {
            if features[4] <= 49.533796_f32 {
                0.644341_f32
            } else {
                if features[14] <= 3.849984_f32 {
                    0.789555_f32
                } else {
                    0.670216_f32
                }
            }
        } else {
            if features[2] <= 11.740656_f32 {
                if features[5] <= 33.162229_f32 {
                    0.948799_f32
                } else {
                    0.800377_f32
                }
            } else {
                if features[14] <= 4.756457_f32 {
                    if features[8] <= 0.101683_f32 {
                        0.747654_f32
                    } else {
                        0.785200_f32
                    }
                } else {
                    if features[8] <= 0.079358_f32 {
                        0.930316_f32
                    } else {
                        0.781322_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_1(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 1.985371_f32 {
        if features[11] <= 2.046558_f32 {
            if features[10] <= 37.338479_f32 {
                if features[10] <= 28.597268_f32 {
                    0.011979_f32
                } else {
                    -0.095898_f32
                }
            } else {
                if features[14] <= 1.519938_f32 {
                    0.113649_f32
                } else {
                    0.066928_f32
                }
            }
        } else {
            if features[0] <= 0.670170_f32 {
                if features[0] <= 0.518319_f32 {
                    if features[0] <= 0.489797_f32 {
                        -0.039462_f32
                    } else {
                        -0.167464_f32
                    }
                } else {
                    if features[7] <= 14.484325_f32 {
                        0.093980_f32
                    } else {
                        -0.055593_f32
                    }
                }
            } else {
                if features[7] <= 13.133877_f32 {
                    if features[14] <= 1.586685_f32 {
                        -0.081761_f32
                    } else {
                        -0.183122_f32
                    }
                } else {
                    -0.015617_f32
                }
            }
        }
    } else {
        if features[7] <= 22.528436_f32 {
            if features[12] <= 93.456601_f32 {
                if features[2] <= 21.840348_f32 {
                    if features[7] <= 16.171516_f32 {
                        -0.011169_f32
                    } else {
                        -0.079042_f32
                    }
                } else {
                    if features[13] <= 5.815137_f32 {
                        -0.146679_f32
                    } else {
                        -0.082531_f32
                    }
                }
            } else {
                -0.163931_f32
            }
        } else {
            if features[1] <= 0.402924_f32 {
                0.066610_f32
            } else {
                if features[10] <= 49.423397_f32 {
                    0.007666_f32
                } else {
                    -0.136928_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_2(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[2] <= 21.840348_f32 {
        if features[6] <= 0.798218_f32 {
            if features[5] <= 16.239368_f32 {
                -0.036798_f32
            } else {
                0.055819_f32
            }
        } else {
            if features[12] <= 92.825489_f32 {
                if features[6] <= 3.014147_f32 {
                    if features[7] <= 22.528436_f32 {
                        -0.042128_f32
                    } else {
                        0.040196_f32
                    }
                } else {
                    if features[6] <= 4.266439_f32 {
                        -0.111179_f32
                    } else {
                        0.025339_f32
                    }
                }
            } else {
                -0.147963_f32
            }
        }
    } else {
        if features[5] <= 23.016209_f32 {
            if features[5] <= 14.596674_f32 {
                if features[5] <= 2.778065_f32 {
                    if features[4] <= 56.322971_f32 {
                        -0.166804_f32
                    } else {
                        -0.056161_f32
                    }
                } else {
                    if features[6] <= 0.705347_f32 {
                        -0.127545_f32
                    } else {
                        -0.042510_f32
                    }
                }
            } else {
                if features[0] <= 0.397124_f32 {
                    if features[10] <= 40.175076_f32 {
                        -0.107532_f32
                    } else {
                        0.018058_f32
                    }
                } else {
                    -0.137969_f32
                }
            }
        } else {
            if features[10] <= 53.043782_f32 {
                if features[1] <= 0.171325_f32 {
                    0.013592_f32
                } else {
                    if features[4] <= 53.712699_f32 {
                        -0.052930_f32
                    } else {
                        -0.092779_f32
                    }
                }
            } else {
                if features[1] <= 0.395604_f32 {
                    0.075142_f32
                } else {
                    if features[7] <= 6.858814_f32 {
                        -0.088113_f32
                    } else {
                        0.041133_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_3(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 1.985371_f32 {
        if features[5] <= 25.860913_f32 {
            if features[11] <= 3.433234_f32 {
                if features[14] <= 1.629720_f32 {
                    if features[10] <= 45.822155_f32 {
                        -0.078101_f32
                    } else {
                        0.047814_f32
                    }
                } else {
                    if features[14] <= 1.870767_f32 {
                        0.114505_f32
                    } else {
                        -0.012157_f32
                    }
                }
            } else {
                if features[11] <= 4.518059_f32 {
                    -0.127058_f32
                } else {
                    -0.010302_f32
                }
            }
        } else {
            if features[13] <= 6.570718_f32 {
                if features[8] <= 0.277851_f32 {
                    0.018880_f32
                } else {
                    0.112576_f32
                }
            } else {
                if features[7] <= 4.219358_f32 {
                    -0.071354_f32
                } else {
                    if features[13] <= 7.374752_f32 {
                        0.073615_f32
                    } else {
                        -0.049073_f32
                    }
                }
            }
        }
    } else {
        if features[5] <= 2.558450_f32 {
            if features[10] <= 46.919190_f32 {
                -0.088486_f32
            } else {
                -0.163528_f32
            }
        } else {
            if features[14] <= 4.756457_f32 {
                if features[8] <= 0.101683_f32 {
                    if features[13] <= 8.073638_f32 {
                        -0.076853_f32
                    } else {
                        -0.142508_f32
                    }
                } else {
                    if features[5] <= 12.294200_f32 {
                        -0.001727_f32
                    } else {
                        -0.062469_f32
                    }
                }
            } else {
                if features[8] <= 0.079358_f32 {
                    0.072574_f32
                } else {
                    if features[14] <= 4.876238_f32 {
                        0.033731_f32
                    } else {
                        -0.084936_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_4(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 1.840385_f32 {
        if features[11] <= 2.046558_f32 {
            if features[10] <= 37.338479_f32 {
                0.000977_f32
            } else {
                0.110561_f32
            }
        } else {
            if features[12] <= 55.475978_f32 {
                if features[10] <= 34.878473_f32 {
                    -0.077005_f32
                } else {
                    0.063302_f32
                }
            } else {
                if features[1] <= 0.307521_f32 {
                    -0.009441_f32
                } else {
                    -0.075179_f32
                }
            }
        }
    } else {
        if features[4] <= 76.760762_f32 {
            if features[14] <= 3.120642_f32 {
                if features[2] <= 33.416672_f32 {
                    if features[12] <= 92.825489_f32 {
                        -0.057010_f32
                    } else {
                        -0.178160_f32
                    }
                } else {
                    if features[13] <= 6.324839_f32 {
                        -0.168806_f32
                    } else {
                        -0.083577_f32
                    }
                }
            } else {
                if features[2] <= 12.066318_f32 {
                    0.033050_f32
                } else {
                    if features[6] <= 4.533028_f32 {
                        -0.051079_f32
                    } else {
                        -0.117066_f32
                    }
                }
            }
        } else {
            if features[11] <= 3.628531_f32 {
                if features[2] <= 30.683121_f32 {
                    if features[0] <= 0.527751_f32 {
                        0.121599_f32
                    } else {
                        0.004742_f32
                    }
                } else {
                    if features[0] <= 0.483926_f32 {
                        -0.116508_f32
                    } else {
                        0.067580_f32
                    }
                }
            } else {
                if features[12] <= 74.905154_f32 {
                    if features[11] <= 4.338376_f32 {
                        -0.137062_f32
                    } else {
                        -0.032985_f32
                    }
                } else {
                    0.005387_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_5(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 2.220612_f32 {
        if features[5] <= 25.860913_f32 {
            if features[1] <= 0.676512_f32 {
                if features[1] <= 0.636421_f32 {
                    if features[10] <= 25.931685_f32 {
                        -0.101077_f32
                    } else {
                        -0.029937_f32
                    }
                } else {
                    -0.149288_f32
                }
            } else {
                0.026362_f32
            }
        } else {
            if features[1] <= 0.279130_f32 {
                if features[14] <= 1.459039_f32 {
                    0.005552_f32
                } else {
                    0.117427_f32
                }
            } else {
                if features[10] <= 29.175866_f32 {
                    if features[7] <= 8.946685_f32 {
                        -0.096749_f32
                    } else {
                        0.012367_f32
                    }
                } else {
                    if features[13] <= 6.570718_f32 {
                        0.075493_f32
                    } else {
                        -0.017623_f32
                    }
                }
            }
        }
    } else {
        if features[14] <= 3.120642_f32 {
            if features[4] <= 78.171242_f32 {
                if features[8] <= 0.125200_f32 {
                    if features[0] <= 0.324759_f32 {
                        -0.035992_f32
                    } else {
                        -0.129517_f32
                    }
                } else {
                    if features[14] <= 2.998338_f32 {
                        -0.056897_f32
                    } else {
                        -0.114505_f32
                    }
                }
            } else {
                if features[3] <= 31.509838_f32 {
                    -0.060066_f32
                } else {
                    0.122778_f32
                }
            }
        } else {
            if features[5] <= 2.558450_f32 {
                -0.111432_f32
            } else {
                if features[7] <= 23.010121_f32 {
                    if features[8] <= 1.291698_f32 {
                        -0.039391_f32
                    } else {
                        -0.091873_f32
                    }
                } else {
                    0.055442_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_6(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 1.985371_f32 {
        if features[5] <= 25.860913_f32 {
            if features[11] <= 3.433234_f32 {
                if features[14] <= 1.629720_f32 {
                    if features[1] <= 0.495206_f32 {
                        0.016564_f32
                    } else {
                        -0.078819_f32
                    }
                } else {
                    if features[14] <= 1.870767_f32 {
                        0.111724_f32
                    } else {
                        0.000672_f32
                    }
                }
            } else {
                if features[11] <= 4.518059_f32 {
                    if features[5] <= 18.195749_f32 {
                        -0.074105_f32
                    } else {
                        -0.149737_f32
                    }
                } else {
                    -0.001406_f32
                }
            }
        } else {
            if features[13] <= 6.570718_f32 {
                if features[6] <= 1.833727_f32 {
                    -0.004980_f32
                } else {
                    0.108060_f32
                }
            } else {
                if features[2] <= 30.838671_f32 {
                    if features[6] <= 2.412020_f32 {
                        0.061673_f32
                    } else {
                        -0.053015_f32
                    }
                } else {
                    -0.056735_f32
                }
            }
        }
    } else {
        if features[5] <= 2.558450_f32 {
            if features[4] <= 49.533796_f32 {
                -0.127653_f32
            } else {
                -0.064922_f32
            }
        } else {
            if features[14] <= 4.756457_f32 {
                if features[2] <= 44.284373_f32 {
                    if features[1] <= 0.713271_f32 {
                        -0.041103_f32
                    } else {
                        -0.077577_f32
                    }
                } else {
                    -0.125912_f32
                }
            } else {
                if features[8] <= 0.079358_f32 {
                    0.073301_f32
                } else {
                    if features[14] <= 4.876238_f32 {
                        0.036321_f32
                    } else {
                        -0.067547_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_7(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 2.220612_f32 {
        if features[5] <= 35.513317_f32 {
            if features[11] <= 2.920213_f32 {
                if features[10] <= 50.093917_f32 {
                    if features[11] <= 2.460428_f32 {
                        -0.033851_f32
                    } else {
                        0.061366_f32
                    }
                } else {
                    0.094543_f32
                }
            } else {
                if features[1] <= 0.676512_f32 {
                    if features[3] <= 26.592722_f32 {
                        -0.022723_f32
                    } else {
                        -0.086832_f32
                    }
                } else {
                    0.045735_f32
                }
            }
        } else {
            if features[1] <= 0.341165_f32 {
                0.094087_f32
            } else {
                if features[0] <= 0.441860_f32 {
                    -0.055137_f32
                } else {
                    if features[4] <= 68.377230_f32 {
                        0.089466_f32
                    } else {
                        -0.037122_f32
                    }
                }
            }
        }
    } else {
        if features[14] <= 2.825105_f32 {
            if features[3] <= 39.430182_f32 {
                if features[8] <= 0.006308_f32 {
                    -0.115908_f32
                } else {
                    if features[14] <= 2.625664_f32 {
                        -0.029557_f32
                    } else {
                        -0.099189_f32
                    }
                }
            } else {
                0.020835_f32
            }
        } else {
            if features[5] <= 6.097790_f32 {
                if features[4] <= 49.533796_f32 {
                    -0.112095_f32
                } else {
                    -0.046323_f32
                }
            } else {
                if features[5] <= 9.667882_f32 {
                    if features[9] <= 0.000000_f32 {
                        -0.020460_f32
                    } else {
                        0.088403_f32
                    }
                } else {
                    if features[5] <= 21.669326_f32 {
                        -0.062211_f32
                    } else {
                        -0.026299_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_8(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[2] <= 21.840348_f32 {
        if features[13] <= 6.240687_f32 {
            if features[0] <= 0.605080_f32 {
                if features[10] <= 41.718705_f32 {
                    if features[9] <= 0.000000_f32 {
                        -0.014958_f32
                    } else {
                        0.122495_f32
                    }
                } else {
                    if features[11] <= 3.758831_f32 {
                        -0.024727_f32
                    } else {
                        -0.127484_f32
                    }
                }
            } else {
                if features[0] <= 0.775411_f32 {
                    0.126405_f32
                } else {
                    0.022768_f32
                }
            }
        } else {
            if features[1] <= 0.752972_f32 {
                if features[1] <= 0.413155_f32 {
                    if features[2] <= 12.763372_f32 {
                        0.072393_f32
                    } else {
                        -0.025674_f32
                    }
                } else {
                    if features[2] <= 19.848333_f32 {
                        -0.074689_f32
                    } else {
                        0.012615_f32
                    }
                }
            } else {
                0.067050_f32
            }
        }
    } else {
        if features[14] <= 4.910260_f32 {
            if features[14] <= 2.220612_f32 {
                if features[5] <= 41.944469_f32 {
                    if features[1] <= 0.678385_f32 {
                        -0.042087_f32
                    } else {
                        0.029908_f32
                    }
                } else {
                    if features[0] <= 0.504697_f32 {
                        -0.000391_f32
                    } else {
                        0.119056_f32
                    }
                }
            } else {
                if features[1] <= 0.734269_f32 {
                    if features[13] <= 5.838929_f32 {
                        -0.091192_f32
                    } else {
                        -0.039050_f32
                    }
                } else {
                    if features[12] <= 76.816732_f32 {
                        -0.115763_f32
                    } else {
                        -0.037736_f32
                    }
                }
            }
        } else {
            0.045898_f32
        }
    }
}

#[inline(always)]
fn tree_9(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[5] <= 23.016209_f32 {
        if features[2] <= 11.740656_f32 {
            0.059857_f32
        } else {
            if features[5] <= 11.030316_f32 {
                if features[5] <= 6.097790_f32 {
                    if features[0] <= 0.739531_f32 {
                        -0.069099_f32
                    } else {
                        0.010269_f32
                    }
                } else {
                    if features[7] <= 14.882920_f32 {
                        -0.007861_f32
                    } else {
                        0.127286_f32
                    }
                }
            } else {
                if features[12] <= 45.516423_f32 {
                    if features[4] <= 58.750458_f32 {
                        -0.054538_f32
                    } else {
                        0.045968_f32
                    }
                } else {
                    if features[0] <= 0.397124_f32 {
                        -0.039645_f32
                    } else {
                        -0.084233_f32
                    }
                }
            }
        }
    } else {
        if features[1] <= 0.263806_f32 {
            if features[5] <= 43.278260_f32 {
                if features[5] <= 39.431705_f32 {
                    if features[5] <= 33.487388_f32 {
                        -0.007845_f32
                    } else {
                        0.063525_f32
                    }
                } else {
                    -0.075624_f32
                }
            } else {
                if features[5] <= 46.825767_f32 {
                    0.127042_f32
                } else {
                    -0.003442_f32
                }
            }
        } else {
            if features[14] <= 1.685886_f32 {
                if features[3] <= 33.204626_f32 {
                    if features[8] <= 0.357558_f32 {
                        0.009281_f32
                    } else {
                        0.111846_f32
                    }
                } else {
                    -0.047933_f32
                }
            } else {
                if features[14] <= 3.348979_f32 {
                    if features[4] <= 80.485824_f32 {
                        -0.066614_f32
                    } else {
                        0.024444_f32
                    }
                } else {
                    -0.020401_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_10(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 1.840385_f32 {
        if features[11] <= 2.046558_f32 {
            if features[5] <= 20.146559_f32 {
                -0.012056_f32
            } else {
                0.079898_f32
            }
        } else {
            if features[12] <= 55.475978_f32 {
                if features[10] <= 34.878473_f32 {
                    -0.051927_f32
                } else {
                    0.062240_f32
                }
            } else {
                if features[10] <= 48.503653_f32 {
                    -0.021543_f32
                } else {
                    -0.075464_f32
                }
            }
        }
    } else {
        if features[4] <= 58.427374_f32 {
            if features[4] <= 52.938025_f32 {
                if features[1] <= 0.441320_f32 {
                    if features[10] <= 42.369606_f32 {
                        0.011817_f32
                    } else {
                        -0.037395_f32
                    }
                } else {
                    if features[12] <= 81.056449_f32 {
                        -0.063280_f32
                    } else {
                        0.013499_f32
                    }
                }
            } else {
                if features[5] <= 36.425166_f32 {
                    if features[13] <= 7.454308_f32 {
                        -0.043072_f32
                    } else {
                        -0.114129_f32
                    }
                } else {
                    -0.136804_f32
                }
            }
        } else {
            if features[7] <= 12.618089_f32 {
                if features[11] <= 4.152200_f32 {
                    if features[2] <= 43.477716_f32 {
                        -0.013719_f32
                    } else {
                        -0.105304_f32
                    }
                } else {
                    if features[11] <= 4.338376_f32 {
                        -0.133511_f32
                    } else {
                        -0.045243_f32
                    }
                }
            } else {
                if features[1] <= 0.724538_f32 {
                    if features[14] <= 2.796279_f32 {
                        -0.054546_f32
                    } else {
                        0.021254_f32
                    }
                } else {
                    0.123859_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_11(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[12] <= 93.456601_f32 {
        if features[2] <= 21.840348_f32 {
            if features[0] <= 0.190111_f32 {
                if features[13] <= 6.828592_f32 {
                    0.003873_f32
                } else {
                    -0.116634_f32
                }
            } else {
                if features[3] <= 40.503059_f32 {
                    if features[6] <= 3.841741_f32 {
                        -0.003921_f32
                    } else {
                        0.083420_f32
                    }
                } else {
                    -0.068242_f32
                }
            }
        } else {
            if features[11] <= 1.056067_f32 {
                0.088320_f32
            } else {
                if features[12] <= 91.516911_f32 {
                    if features[8] <= 0.536548_f32 {
                        -0.029815_f32
                    } else {
                        -0.062679_f32
                    }
                } else {
                    0.039678_f32
                }
            }
        }
    } else {
        -0.083828_f32
    }
}

#[inline(always)]
fn tree_12(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[10] <= 20.849390_f32 {
        -0.079497_f32
    } else {
        if features[10] <= 21.276607_f32 {
            0.087474_f32
        } else {
            if features[1] <= 0.439071_f32 {
                if features[1] <= 0.424917_f32 {
                    if features[2] <= 12.763372_f32 {
                        0.047934_f32
                    } else {
                        -0.020950_f32
                    }
                } else {
                    0.085027_f32
                }
            } else {
                if features[11] <= 1.960968_f32 {
                    if features[6] <= 3.559705_f32 {
                        0.019184_f32
                    } else {
                        -0.093371_f32
                    }
                } else {
                    if features[11] <= 2.972920_f32 {
                        -0.063967_f32
                    } else {
                        -0.030477_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_13(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 1.194441_f32 {
        if features[4] <= 52.777888_f32 {
            0.072440_f32
        } else {
            if features[14] <= 1.077696_f32 {
                -0.060586_f32
            } else {
                0.045420_f32
            }
        }
    } else {
        if features[4] <= 78.171242_f32 {
            if features[5] <= 2.558450_f32 {
                if features[12] <= 58.168421_f32 {
                    -0.022865_f32
                } else {
                    if features[2] <= 26.384858_f32 {
                        -0.037655_f32
                    } else {
                        -0.111896_f32
                    }
                }
            } else {
                if features[2] <= 44.204866_f32 {
                    if features[12] <= 93.456601_f32 {
                        -0.021805_f32
                    } else {
                        -0.081127_f32
                    }
                } else {
                    -0.083695_f32
                }
            }
        } else {
            if features[11] <= 3.628531_f32 {
                if features[3] <= 36.290643_f32 {
                    if features[5] <= 30.824872_f32 {
                        0.048709_f32
                    } else {
                        -0.028401_f32
                    }
                } else {
                    0.105599_f32
                }
            } else {
                if features[5] <= 31.209263_f32 {
                    if features[14] <= 3.209193_f32 {
                        -0.028810_f32
                    } else {
                        -0.097159_f32
                    }
                } else {
                    0.034584_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_14(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[10] <= 20.849390_f32 {
        -0.070553_f32
    } else {
        if features[14] <= 1.427380_f32 {
            if features[1] <= 0.685180_f32 {
                if features[13] <= 7.362895_f32 {
                    if features[1] <= 0.594922_f32 {
                        0.047489_f32
                    } else {
                        -0.059445_f32
                    }
                } else {
                    if features[6] <= 2.412020_f32 {
                        0.001840_f32
                    } else {
                        -0.093164_f32
                    }
                }
            } else {
                0.082408_f32
            }
        } else {
            if features[10] <= 21.276607_f32 {
                0.135382_f32
            } else {
                if features[4] <= 58.427374_f32 {
                    if features[4] <= 56.169011_f32 {
                        -0.026391_f32
                    } else {
                        -0.081660_f32
                    }
                } else {
                    if features[13] <= 5.691085_f32 {
                        -0.071289_f32
                    } else {
                        -0.007298_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_15(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[5] <= 33.648645_f32 {
        if features[10] <= 20.849390_f32 {
            -0.093459_f32
        } else {
            if features[11] <= 3.867279_f32 {
                if features[6] <= 0.525420_f32 {
                    0.069656_f32
                } else {
                    if features[8] <= 0.985013_f32 {
                        -0.025679_f32
                    } else {
                        0.012923_f32
                    }
                }
            } else {
                if features[7] <= 19.985700_f32 {
                    if features[11] <= 4.502551_f32 {
                        -0.062790_f32
                    } else {
                        -0.020462_f32
                    }
                } else {
                    if features[12] <= 65.549838_f32 {
                        -0.000495_f32
                    } else {
                        0.061817_f32
                    }
                }
            }
        }
    } else {
        if features[0] <= 0.238598_f32 {
            if features[6] <= 1.871146_f32 {
                if features[14] <= 3.332056_f32 {
                    -0.065284_f32
                } else {
                    0.059202_f32
                }
            } else {
                if features[5] <= 40.942677_f32 {
                    -0.028304_f32
                } else {
                    -0.107155_f32
                }
            }
        } else {
            if features[0] <= 0.307847_f32 {
                if features[2] <= 33.416672_f32 {
                    0.129692_f32
                } else {
                    0.030898_f32
                }
            } else {
                if features[11] <= 2.934486_f32 {
                    if features[11] <= 2.474923_f32 {
                        -0.010490_f32
                    } else {
                        -0.073988_f32
                    }
                } else {
                    if features[4] <= 76.014651_f32 {
                        0.003920_f32
                    } else {
                        0.074701_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_16(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[12] <= 93.456601_f32 {
        if features[5] <= 2.558450_f32 {
            if features[14] <= 3.643830_f32 {
                if features[12] <= 81.651323_f32 {
                    if features[10] <= 46.057144_f32 {
                        0.044636_f32
                    } else {
                        -0.040123_f32
                    }
                } else {
                    -0.085156_f32
                }
            } else {
                -0.087119_f32
            }
        } else {
            if features[12] <= 76.654468_f32 {
                if features[1] <= 0.554286_f32 {
                    if features[2] <= 44.204866_f32 {
                        -0.006411_f32
                    } else {
                        -0.096289_f32
                    }
                } else {
                    if features[1] <= 0.652397_f32 {
                        -0.059799_f32
                    } else {
                        -0.019158_f32
                    }
                }
            } else {
                if features[1] <= 0.679851_f32 {
                    if features[4] <= 64.030795_f32 {
                        -0.023027_f32
                    } else {
                        0.027146_f32
                    }
                } else {
                    if features[2] <= 31.856428_f32 {
                        0.099476_f32
                    } else {
                        0.000133_f32
                    }
                }
            }
        }
    } else {
        -0.064431_f32
    }
}

#[inline(always)]
fn tree_17(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[0] <= 0.872582_f32 {
        if features[0] <= 0.863192_f32 {
            if features[12] <= 92.825489_f32 {
                if features[2] <= 11.393150_f32 {
                    if features[11] <= 3.210688_f32 {
                        -0.011806_f32
                    } else {
                        0.113203_f32
                    }
                } else {
                    if features[12] <= 74.517451_f32 {
                        -0.021909_f32
                    } else {
                        -0.002280_f32
                    }
                }
            } else {
                if features[6] <= 2.115377_f32 {
                    -0.087939_f32
                } else {
                    0.036063_f32
                }
            }
        } else {
            -0.084304_f32
        }
    } else {
        if features[12] <= 76.184035_f32 {
            0.063858_f32
        } else {
            -0.020571_f32
        }
    }
}

#[inline(always)]
fn tree_18(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[5] <= 23.016209_f32 {
        if features[2] <= 11.740656_f32 {
            0.057686_f32
        } else {
            if features[5] <= 11.030316_f32 {
                if features[5] <= 6.097790_f32 {
                    if features[5] <= 5.528014_f32 {
                        -0.023002_f32
                    } else {
                        -0.106410_f32
                    }
                } else {
                    if features[7] <= 14.882920_f32 {
                        0.008151_f32
                    } else {
                        0.127861_f32
                    }
                }
            } else {
                if features[13] <= 7.103067_f32 {
                    -0.051741_f32
                } else {
                    if features[1] <= 0.460479_f32 {
                        0.017833_f32
                    } else {
                        -0.042282_f32
                    }
                }
            }
        }
    } else {
        if features[1] <= 0.263806_f32 {
            if features[5] <= 43.278260_f32 {
                if features[13] <= 7.623214_f32 {
                    if features[11] <= 4.046989_f32 {
                        0.005685_f32
                    } else {
                        0.126827_f32
                    }
                } else {
                    -0.046884_f32
                }
            } else {
                if features[5] <= 46.825767_f32 {
                    0.126776_f32
                } else {
                    0.005170_f32
                }
            }
        } else {
            if features[0] <= 0.249255_f32 {
                if features[1] <= 0.626024_f32 {
                    -0.063333_f32
                } else {
                    if features[8] <= 0.728643_f32 {
                        0.045728_f32
                    } else {
                        -0.075860_f32
                    }
                }
            } else {
                if features[0] <= 0.306338_f32 {
                    if features[0] <= 0.272598_f32 {
                        0.011088_f32
                    } else {
                        0.136581_f32
                    }
                } else {
                    if features[11] <= 1.393125_f32 {
                        -0.061825_f32
                    } else {
                        -0.006071_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_19(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 2.220612_f32 {
        if features[11] <= 2.920213_f32 {
            if features[11] <= 2.714720_f32 {
                if features[10] <= 50.093917_f32 {
                    if features[2] <= 36.475756_f32 {
                        -0.033425_f32
                    } else {
                        0.059239_f32
                    }
                } else {
                    if features[3] <= 27.710079_f32 {
                        0.018497_f32
                    } else {
                        0.118115_f32
                    }
                }
            } else {
                0.126498_f32
            }
        } else {
            if features[11] <= 3.086444_f32 {
                -0.086311_f32
            } else {
                if features[2] <= 42.081790_f32 {
                    if features[11] <= 3.433234_f32 {
                        0.075189_f32
                    } else {
                        -0.009526_f32
                    }
                } else {
                    -0.079057_f32
                }
            }
        }
    } else {
        if features[14] <= 2.863004_f32 {
            if features[3] <= 35.950688_f32 {
                if features[0] <= 0.208208_f32 {
                    -0.095596_f32
                } else {
                    if features[14] <= 2.768709_f32 {
                        -0.027363_f32
                    } else {
                        -0.099404_f32
                    }
                }
            } else {
                if features[9] <= 0.000000_f32 {
                    0.076724_f32
                } else {
                    -0.042280_f32
                }
            }
        } else {
            if features[11] <= 4.732218_f32 {
                if features[11] <= 4.481266_f32 {
                    if features[10] <= 23.205485_f32 {
                        -0.055154_f32
                    } else {
                        -0.008199_f32
                    }
                } else {
                    if features[14] <= 4.214981_f32 {
                        0.099555_f32
                    } else {
                        -0.001912_f32
                    }
                }
            } else {
                if features[3] <= 32.950678_f32 {
                    -0.089143_f32
                } else {
                    -0.001611_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_20(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[5] <= 23.016209_f32 {
        if features[2] <= 11.740656_f32 {
            0.055619_f32
        } else {
            if features[1] <= 0.139748_f32 {
                if features[10] <= 39.122303_f32 {
                    -0.109630_f32
                } else {
                    -0.011998_f32
                }
            } else {
                if features[5] <= 11.030316_f32 {
                    if features[5] <= 6.097790_f32 {
                        -0.025608_f32
                    } else {
                        0.029833_f32
                    }
                } else {
                    if features[13] <= 7.567204_f32 {
                        -0.040276_f32
                    } else {
                        0.002350_f32
                    }
                }
            }
        }
    } else {
        if features[13] <= 7.416331_f32 {
            if features[1] <= 0.294361_f32 {
                if features[7] <= 11.704962_f32 {
                    if features[13] <= 6.701189_f32 {
                        -0.026905_f32
                    } else {
                        0.072520_f32
                    }
                } else {
                    0.081483_f32
                }
            } else {
                if features[8] <= 2.283553_f32 {
                    if features[3] <= 32.200915_f32 {
                        -0.015754_f32
                    } else {
                        0.020837_f32
                    }
                } else {
                    -0.093962_f32
                }
            }
        } else {
            if features[12] <= 76.051737_f32 {
                if features[12] <= 62.350008_f32 {
                    if features[11] <= 2.682925_f32 {
                        -0.062315_f32
                    } else {
                        0.013688_f32
                    }
                } else {
                    if features[13] <= 7.814320_f32 {
                        -0.114762_f32
                    } else {
                        -0.038539_f32
                    }
                }
            } else {
                if features[10] <= 46.305449_f32 {
                    if features[12] <= 89.620908_f32 {
                        0.016505_f32
                    } else {
                        -0.078428_f32
                    }
                } else {
                    0.109908_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_21(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[14] <= 1.985371_f32 {
        if features[13] <= 5.932073_f32 {
            if features[11] <= 4.031317_f32 {
                0.093567_f32
            } else {
                -0.020426_f32
            }
        } else {
            if features[10] <= 25.931685_f32 {
                if features[10] <= 24.174422_f32 {
                    0.002826_f32
                } else {
                    -0.091635_f32
                }
            } else {
                if features[11] <= 2.164962_f32 {
                    if features[11] <= 1.257747_f32 {
                        -0.013860_f32
                    } else {
                        0.099940_f32
                    }
                } else {
                    if features[12] <= 62.562603_f32 {
                        0.024848_f32
                    } else {
                        -0.029578_f32
                    }
                }
            }
        }
    } else {
        if features[3] <= 41.253109_f32 {
            if features[14] <= 2.056833_f32 {
                -0.070246_f32
            } else {
                if features[14] <= 2.162343_f32 {
                    0.048963_f32
                } else {
                    if features[4] <= 58.427374_f32 {
                        -0.021925_f32
                    } else {
                        -0.003042_f32
                    }
                }
            }
        } else {
            -0.079107_f32
        }
    }
}

#[inline(always)]
fn tree_22(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[6] <= 0.315798_f32 {
        -0.078726_f32
    } else {
        if features[4] <= 83.915678_f32 {
            if features[6] <= 0.445527_f32 {
                0.134084_f32
            } else {
                if features[4] <= 82.339434_f32 {
                    if features[4] <= 80.318481_f32 {
                        -0.012951_f32
                    } else {
                        0.049677_f32
                    }
                } else {
                    -0.075728_f32
                }
            }
        } else {
            if features[10] <= 47.748916_f32 {
                if features[13] <= 7.695972_f32 {
                    if features[3] <= 30.914588_f32 {
                        0.077357_f32
                    } else {
                        -0.025490_f32
                    }
                } else {
                    -0.068172_f32
                }
            } else {
                0.127104_f32
            }
        }
    }
}

#[inline(always)]
fn tree_23(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[0] <= 0.872582_f32 {
        if features[0] <= 0.863192_f32 {
            if features[1] <= 0.261085_f32 {
                if features[13] <= 5.815137_f32 {
                    -0.061483_f32
                } else {
                    if features[4] <= 60.996832_f32 {
                        -0.008001_f32
                    } else {
                        0.052562_f32
                    }
                }
            } else {
                if features[13] <= 7.416331_f32 {
                    if features[12] <= 85.148382_f32 {
                        -0.002602_f32
                    } else {
                        -0.039483_f32
                    }
                } else {
                    if features[13] <= 7.567204_f32 {
                        -0.075710_f32
                    } else {
                        -0.011875_f32
                    }
                }
            }
        } else {
            -0.074959_f32
        }
    } else {
        if features[4] <= 60.285743_f32 {
            if features[5] <= 32.079757_f32 {
                -0.026228_f32
            } else {
                0.039916_f32
            }
        } else {
            0.091088_f32
        }
    }
}

#[inline(always)]
fn tree_24(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[5] <= 2.558450_f32 {
        if features[14] <= 3.849984_f32 {
            if features[8] <= 0.635539_f32 {
                if features[1] <= 0.498621_f32 {
                    -0.014372_f32
                } else {
                    0.047837_f32
                }
            } else {
                -0.074199_f32
            }
        } else {
            -0.074204_f32
        }
    } else {
        if features[5] <= 4.390043_f32 {
            if features[11] <= 3.254592_f32 {
                0.017549_f32
            } else {
                0.094177_f32
            }
        } else {
            if features[5] <= 6.097790_f32 {
                if features[1] <= 0.319911_f32 {
                    -0.091914_f32
                } else {
                    -0.020428_f32
                }
            } else {
                if features[5] <= 11.030316_f32 {
                    if features[7] <= 14.882920_f32 {
                        0.009740_f32
                    } else {
                        0.121962_f32
                    }
                } else {
                    if features[5] <= 22.196541_f32 {
                        -0.025837_f32
                    } else {
                        -0.003131_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_25(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[12] <= 93.456601_f32 {
        if features[12] <= 76.654468_f32 {
            if features[1] <= 0.439071_f32 {
                if features[1] <= 0.418478_f32 {
                    if features[0] <= 0.489797_f32 {
                        0.012604_f32
                    } else {
                        -0.018060_f32
                    }
                } else {
                    0.093287_f32
                }
            } else {
                if features[6] <= 2.680549_f32 {
                    if features[3] <= 38.854802_f32 {
                        -0.017220_f32
                    } else {
                        0.091081_f32
                    }
                } else {
                    if features[3] <= 33.693266_f32 {
                        -0.086248_f32
                    } else {
                        -0.019557_f32
                    }
                }
            }
        } else {
            if features[12] <= 77.536344_f32 {
                0.092040_f32
            } else {
                if features[5] <= 6.528847_f32 {
                    if features[14] <= 3.905412_f32 {
                        -0.011992_f32
                    } else {
                        -0.089455_f32
                    }
                } else {
                    if features[6] <= 3.970222_f32 {
                        0.013113_f32
                    } else {
                        -0.056623_f32
                    }
                }
            }
        }
    } else {
        -0.048438_f32
    }
}

#[inline(always)]
fn tree_26(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[6] <= 0.315798_f32 {
        -0.069741_f32
    } else {
        if features[4] <= 83.915678_f32 {
            if features[6] <= 0.445527_f32 {
                0.128805_f32
            } else {
                if features[4] <= 82.339434_f32 {
                    if features[4] <= 80.318481_f32 {
                        -0.009618_f32
                    } else {
                        0.047534_f32
                    }
                } else {
                    -0.067553_f32
                }
            }
        } else {
            if features[4] <= 88.658720_f32 {
                if features[14] <= 3.860273_f32 {
                    if features[0] <= 0.369400_f32 {
                        0.007542_f32
                    } else {
                        0.110954_f32
                    }
                } else {
                    -0.003780_f32
                }
            } else {
                -0.081512_f32
            }
        }
    }
}

#[inline(always)]
fn tree_27(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[10] <= 20.849390_f32 {
        -0.046860_f32
    } else {
        if features[14] <= 1.427380_f32 {
            if features[12] <= 47.129381_f32 {
                0.079685_f32
            } else {
                if features[5] <= 21.181974_f32 {
                    if features[5] <= 14.053582_f32 {
                        0.050842_f32
                    } else {
                        -0.095248_f32
                    }
                } else {
                    if features[3] <= 33.204626_f32 {
                        0.062945_f32
                    } else {
                        -0.025853_f32
                    }
                }
            }
        } else {
            if features[10] <= 21.276607_f32 {
                0.133943_f32
            } else {
                if features[14] <= 4.756457_f32 {
                    if features[7] <= 1.576472_f32 {
                        -0.022728_f32
                    } else {
                        -0.004854_f32
                    }
                } else {
                    if features[2] <= 36.059599_f32 {
                        -0.003848_f32
                    } else {
                        0.100513_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_28(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[0] <= 0.872582_f32 {
        if features[0] <= 0.863192_f32 {
            if features[1] <= 0.261085_f32 {
                if features[13] <= 5.709677_f32 {
                    -0.068069_f32
                } else {
                    if features[4] <= 60.996832_f32 {
                        -0.004541_f32
                    } else {
                        0.051915_f32
                    }
                }
            } else {
                if features[1] <= 0.304288_f32 {
                    if features[10] <= 51.561577_f32 {
                        -0.056287_f32
                    } else {
                        0.035075_f32
                    }
                } else {
                    if features[14] <= 4.764161_f32 {
                        -0.009742_f32
                    } else {
                        0.032592_f32
                    }
                }
            }
        } else {
            -0.066970_f32
        }
    } else {
        if features[13] <= 6.465180_f32 {
            -0.015644_f32
        } else {
            0.058464_f32
        }
    }
}

#[inline(always)]
fn tree_29(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[12] <= 93.456601_f32 {
        if features[13] <= 8.120718_f32 {
            if features[1] <= 0.439071_f32 {
                if features[13] <= 5.673264_f32 {
                    -0.058056_f32
                } else {
                    if features[0] <= 0.489797_f32 {
                        0.023292_f32
                    } else {
                        -0.004868_f32
                    }
                }
            } else {
                if features[12] <= 75.864297_f32 {
                    if features[12] <= 45.244936_f32 {
                        0.006327_f32
                    } else {
                        -0.029763_f32
                    }
                } else {
                    if features[9] <= 0.000000_f32 {
                        0.040700_f32
                    } else {
                        -0.008320_f32
                    }
                }
            }
        } else {
            if features[9] <= 0.000000_f32 {
                -0.062498_f32
            } else {
                0.004704_f32
            }
        }
    } else {
        -0.042786_f32
    }
}

#[inline(always)]
fn tree_30(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[5] <= 2.558450_f32 {
        if features[10] <= 46.919190_f32 {
            if features[3] <= 31.586538_f32 {
                0.035031_f32
            } else {
                -0.054812_f32
            }
        } else {
            -0.069698_f32
        }
    } else {
        if features[5] <= 4.390043_f32 {
            if features[14] <= 3.702107_f32 {
                0.075468_f32
            } else {
                0.017221_f32
            }
        } else {
            if features[5] <= 6.097790_f32 {
                if features[10] <= 40.175076_f32 {
                    -0.071385_f32
                } else {
                    -0.003675_f32
                }
            } else {
                if features[5] <= 11.030316_f32 {
                    if features[7] <= 14.882920_f32 {
                        0.011880_f32
                    } else {
                        0.118317_f32
                    }
                } else {
                    if features[5] <= 22.196541_f32 {
                        -0.020653_f32
                    } else {
                        -0.000421_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_31(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[10] <= 20.849390_f32 {
        -0.041972_f32
    } else {
        if features[10] <= 21.276607_f32 {
            0.082503_f32
        } else {
            if features[14] <= 1.194441_f32 {
                if features[10] <= 32.610383_f32 {
                    0.091667_f32
                } else {
                    if features[14] <= 1.077696_f32 {
                        -0.025694_f32
                    } else {
                        0.054169_f32
                    }
                }
            } else {
                if features[13] <= 8.046192_f32 {
                    if features[13] <= 7.993712_f32 {
                        -0.005425_f32
                    } else {
                        0.052946_f32
                    }
                } else {
                    if features[2] <= 40.335109_f32 {
                        -0.015376_f32
                    } else {
                        -0.076887_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_32(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[6] <= 0.315798_f32 {
        -0.062136_f32
    } else {
        if features[4] <= 83.915678_f32 {
            if features[6] <= 0.445527_f32 {
                0.124936_f32
            } else {
                if features[4] <= 82.339434_f32 {
                    if features[4] <= 80.318481_f32 {
                        -0.006420_f32
                    } else {
                        0.045324_f32
                    }
                } else {
                    -0.059186_f32
                }
            }
        } else {
            if features[4] <= 88.658720_f32 {
                if features[14] <= 3.860273_f32 {
                    if features[0] <= 0.349415_f32 {
                        0.004570_f32
                    } else {
                        0.107218_f32
                    }
                } else {
                    -0.002358_f32
                }
            } else {
                -0.072924_f32
            }
        }
    }
}

#[inline(always)]
fn tree_33(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[12] <= 93.456601_f32 {
        if features[5] <= 2.558450_f32 {
            if features[12] <= 82.631682_f32 {
                if features[10] <= 46.057144_f32 {
                    if features[5] <= 1.349479_f32 {
                        0.074262_f32
                    } else {
                        -0.007666_f32
                    }
                } else {
                    -0.053032_f32
                }
            } else {
                -0.084934_f32
            }
        } else {
            if features[5] <= 4.390043_f32 {
                0.052316_f32
            } else {
                if features[5] <= 6.097790_f32 {
                    if features[6] <= 2.115377_f32 {
                        -0.011240_f32
                    } else {
                        -0.084900_f32
                    }
                } else {
                    if features[5] <= 11.030316_f32 {
                        0.028352_f32
                    } else {
                        -0.004297_f32
                    }
                }
            }
        }
    } else {
        -0.038787_f32
    }
}

#[inline(always)]
fn tree_34(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[2] <= 43.180975_f32 {
        if features[2] <= 42.485272_f32 {
            if features[4] <= 21.043213_f32 {
                -0.050941_f32
            } else {
                if features[4] <= 25.841937_f32 {
                    if features[14] <= 2.698554_f32 {
                        0.069510_f32
                    } else {
                        0.005387_f32
                    }
                } else {
                    if features[4] <= 29.360987_f32 {
                        -0.033337_f32
                    } else {
                        -0.003044_f32
                    }
                }
            }
        } else {
            0.069495_f32
        }
    } else {
        if features[4] <= 50.977458_f32 {
            0.023649_f32
        } else {
            if features[5] <= 23.562051_f32 {
                -0.114054_f32
            } else {
                -0.001755_f32
            }
        }
    }
}

#[inline(always)]
fn tree_35(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[0] <= 0.872582_f32 {
        if features[0] <= 0.863192_f32 {
            if features[10] <= 20.849390_f32 {
                -0.039340_f32
            } else {
                if features[14] <= 1.439031_f32 {
                    if features[14] <= 1.329393_f32 {
                        0.004788_f32
                    } else {
                        0.060385_f32
                    }
                } else {
                    if features[0] <= 0.414237_f32 {
                        0.003704_f32
                    } else {
                        -0.010941_f32
                    }
                }
            }
        } else {
            -0.058607_f32
        }
    } else {
        if features[12] <= 76.184035_f32 {
            0.061223_f32
        } else {
            -0.013394_f32
        }
    }
}

#[inline(always)]
fn tree_36(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[6] <= 0.315798_f32 {
        -0.057848_f32
    } else {
        if features[4] <= 83.915678_f32 {
            if features[6] <= 0.445527_f32 {
                0.121341_f32
            } else {
                if features[6] <= 1.010745_f32 {
                    if features[2] <= 41.546679_f32 {
                        -0.011121_f32
                    } else {
                        -0.091773_f32
                    }
                } else {
                    if features[4] <= 77.405639_f32 {
                        -0.003608_f32
                    } else {
                        0.056665_f32
                    }
                }
            }
        } else {
            if features[10] <= 47.748916_f32 {
                if features[12] <= 82.631682_f32 {
                    if features[11] <= 3.867279_f32 {
                        0.059735_f32
                    } else {
                        -0.050106_f32
                    }
                } else {
                    -0.069497_f32
                }
            } else {
                0.117110_f32
            }
        }
    }
}

#[inline(always)]
fn tree_37(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[1] <= 0.117558_f32 {
        if features[3] <= 33.951421_f32 {
            -0.069219_f32
        } else {
            0.015288_f32
        }
    } else {
        if features[1] <= 0.121344_f32 {
            0.140251_f32
        } else {
            if features[0] <= 0.872582_f32 {
                if features[1] <= 0.441320_f32 {
                    if features[13] <= 5.660034_f32 {
                        -0.065332_f32
                    } else {
                        0.007789_f32
                    }
                } else {
                    if features[11] <= 1.960968_f32 {
                        0.018359_f32
                    } else {
                        -0.017185_f32
                    }
                }
            } else {
                if features[13] <= 6.465180_f32 {
                    -0.015651_f32
                } else {
                    0.067071_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_38(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[2] <= 24.688549_f32 {
        if features[6] <= 0.789858_f32 {
            if features[5] <= 16.239368_f32 {
                -0.006871_f32
            } else {
                if features[13] <= 7.148197_f32 {
                    0.105268_f32
                } else {
                    0.029221_f32
                }
            }
        } else {
            if features[6] <= 0.860317_f32 {
                -0.067612_f32
            } else {
                if features[0] <= 0.199584_f32 {
                    if features[13] <= 7.103067_f32 {
                        0.006434_f32
                    } else {
                        -0.064765_f32
                    }
                } else {
                    if features[12] <= 54.344237_f32 {
                        0.032332_f32
                    } else {
                        -0.001577_f32
                    }
                }
            }
        }
    } else {
        if features[8] <= 0.516559_f32 {
            if features[3] <= 33.879918_f32 {
                if features[14] <= 4.671106_f32 {
                    if features[2] <= 30.262466_f32 {
                        0.015079_f32
                    } else {
                        -0.023429_f32
                    }
                } else {
                    if features[6] <= 1.444567_f32 {
                        -0.001225_f32
                    } else {
                        0.102315_f32
                    }
                }
            } else {
                if features[6] <= 4.533028_f32 {
                    if features[6] <= 3.431040_f32 {
                        0.018554_f32
                    } else {
                        0.086389_f32
                    }
                } else {
                    -0.053576_f32
                }
            }
        } else {
            if features[3] <= 38.786889_f32 {
                if features[0] <= 0.507113_f32 {
                    -0.020191_f32
                } else {
                    -0.094497_f32
                }
            } else {
                -0.002735_f32
            }
        }
    }
}

#[inline(always)]
fn tree_39(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[11] <= 1.377705_f32 {
        if features[3] <= 26.367544_f32 {
            if features[3] <= 22.956895_f32 {
                if features[4] <= 55.361541_f32 {
                    -0.057419_f32
                } else {
                    0.047757_f32
                }
            } else {
                -0.097876_f32
            }
        } else {
            if features[6] <= 2.861364_f32 {
                if features[10] <= 50.923152_f32 {
                    if features[11] <= 1.154902_f32 {
                        0.140583_f32
                    } else {
                        0.021691_f32
                    }
                } else {
                    -0.040870_f32
                }
            } else {
                -0.053007_f32
            }
        }
    } else {
        if features[11] <= 1.851344_f32 {
            if features[13] <= 5.986383_f32 {
                -0.028322_f32
            } else {
                if features[13] <= 7.119653_f32 {
                    if features[7] <= 14.569240_f32 {
                        0.128780_f32
                    } else {
                        0.017696_f32
                    }
                } else {
                    if features[14] <= 4.329161_f32 {
                        0.047791_f32
                    } else {
                        -0.060461_f32
                    }
                }
            }
        } else {
            if features[1] <= 0.439071_f32 {
                if features[1] <= 0.418478_f32 {
                    if features[3] <= 37.812277_f32 {
                        0.009463_f32
                    } else {
                        -0.029700_f32
                    }
                } else {
                    0.074976_f32
                }
            } else {
                if features[5] <= 33.648645_f32 {
                    if features[12] <= 75.591714_f32 {
                        -0.029767_f32
                    } else {
                        0.000151_f32
                    }
                } else {
                    if features[11] <= 3.056258_f32 {
                        -0.036545_f32
                    } else {
                        0.034589_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_40(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[10] <= 20.849390_f32 {
        -0.034753_f32
    } else {
        if features[10] <= 21.276607_f32 {
            0.081477_f32
        } else {
            if features[14] <= 1.194441_f32 {
                if features[4] <= 71.660200_f32 {
                    if features[13] <= 7.362895_f32 {
                        0.116754_f32
                    } else {
                        0.006247_f32
                    }
                } else {
                    -0.029206_f32
                }
            } else {
                if features[8] <= 2.569556_f32 {
                    if features[8] <= 1.935877_f32 {
                        -0.002377_f32
                    } else {
                        -0.053089_f32
                    }
                } else {
                    0.051155_f32
                }
            }
        }
    }
}

#[inline(always)]
fn tree_41(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[12] <= 93.456601_f32 {
        if features[12] <= 76.654468_f32 {
            if features[12] <= 68.394533_f32 {
                if features[0] <= 0.249255_f32 {
                    if features[1] <= 0.554286_f32 {
                        -0.004154_f32
                    } else {
                        -0.055474_f32
                    }
                } else {
                    if features[7] <= 10.327402_f32 {
                        -0.001985_f32
                    } else {
                        0.024944_f32
                    }
                }
            } else {
                if features[10] <= 40.058869_f32 {
                    if features[0] <= 0.760447_f32 {
                        0.024154_f32
                    } else {
                        -0.086988_f32
                    }
                } else {
                    if features[12] <= 72.051917_f32 {
                        -0.093467_f32
                    } else {
                        -0.020270_f32
                    }
                }
            }
        } else {
            if features[1] <= 0.679851_f32 {
                if features[1] <= 0.143404_f32 {
                    -0.051310_f32
                } else {
                    if features[5] <= 6.528847_f32 {
                        -0.041907_f32
                    } else {
                        0.015421_f32
                    }
                }
            } else {
                if features[8] <= 0.272396_f32 {
                    0.002287_f32
                } else {
                    if features[7] <= 16.434629_f32 {
                        0.135896_f32
                    } else {
                        0.064261_f32
                    }
                }
            }
        }
    } else {
        -0.033998_f32
    }
}

#[inline(always)]
fn tree_42(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[0] <= 0.411399_f32 {
        if features[5] <= 2.019150_f32 {
            -0.049675_f32
        } else {
            if features[3] <= 20.067023_f32 {
                if features[5] <= 19.170990_f32 {
                    -0.075676_f32
                } else {
                    0.015494_f32
                }
            } else {
                if features[0] <= 0.249255_f32 {
                    if features[5] <= 18.944665_f32 {
                        0.047831_f32
                    } else {
                        -0.021331_f32
                    }
                } else {
                    if features[2] <= 29.554313_f32 {
                        0.060906_f32
                    } else {
                        0.002132_f32
                    }
                }
            }
        }
    } else {
        if features[11] <= 1.393125_f32 {
            if features[0] <= 0.741570_f32 {
                if features[11] <= 1.191769_f32 {
                    -0.019617_f32
                } else {
                    -0.084984_f32
                }
            } else {
                0.016383_f32
            }
        } else {
            if features[11] <= 1.960968_f32 {
                if features[1] <= 0.433176_f32 {
                    if features[0] <= 0.695754_f32 {
                        -0.058018_f32
                    } else {
                        0.073992_f32
                    }
                } else {
                    if features[5] <= 21.796406_f32 {
                        0.054102_f32
                    } else {
                        0.118379_f32
                    }
                }
            } else {
                if features[3] <= 18.947701_f32 {
                    0.063563_f32
                } else {
                    if features[7] <= 13.387204_f32 {
                        -0.018146_f32
                    } else {
                        0.009498_f32
                    }
                }
            }
        }
    }
}

#[inline(always)]
fn tree_43(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[8] <= 1.935877_f32 {
        if features[7] <= 22.528436_f32 {
            if features[2] <= 12.544509_f32 {
                -0.117616_f32
            } else {
                if features[2] <= 12.763372_f32 {
                    0.128619_f32
                } else {
                    if features[2] <= 12.975397_f32 {
                        -0.082545_f32
                    } else {
                        -0.001639_f32
                    }
                }
            }
        } else {
            if features[3] <= 26.367544_f32 {
                if features[13] <= 7.406767_f32 {
                    0.040136_f32
                } else {
                    -0.066998_f32
                }
            } else {
                if features[14] <= 4.072897_f32 {
                    0.114842_f32
                } else {
                    0.068154_f32
                }
            }
        }
    } else {
        if features[6] <= 4.211475_f32 {
            -0.062683_f32
        } else {
            0.030114_f32
        }
    }
}

#[inline(always)]
fn tree_44(features: &[f32; NUM_FEATURES]) -> f32 {
    if features[1] <= 0.776642_f32 {
        if features[1] <= 0.652397_f32 {
            if features[1] <= 0.439071_f32 {
                if features[13] <= 5.709677_f32 {
                    if features[6] <= 1.925396_f32 {
                        -0.072698_f32
                    } else {
                        0.028903_f32
                    }
                } else {
                    if features[3] <= 41.253109_f32 {
                        0.009169_f32
                    } else {
                        -0.075056_f32
                    }
                }
            } else {
                if features[4] <= 46.230307_f32 {
                    if features[8] <= 1.141563_f32 {
                        -0.048595_f32
                    } else {
                        0.011914_f32
                    }
                } else {
                    if features[3] <= 38.786889_f32 {
                        -0.010176_f32
                    } else {
                        0.093863_f32
                    }
                }
            }
        } else {
            if features[1] <= 0.701514_f32 {
                if features[7] <= 4.639482_f32 {
                    0.101973_f32
                } else {
                    if features[1] <= 0.679851_f32 {
                        -0.014105_f32
                    } else {
                        0.057709_f32
                    }
                }
            } else {
                if features[7] <= 8.059418_f32 {
                    if features[10] <= 49.423397_f32 {
                        -0.003683_f32
                    } else {
                        -0.071480_f32
                    }
                } else {
                    if features[8] <= 1.034241_f32 {
                        0.081275_f32
                    } else {
                        -0.048779_f32
                    }
                }
            }
        }
    } else {
        if features[1] <= 0.786792_f32 {
            -0.061769_f32
        } else {
            0.022629_f32
        }
    }
}

pub fn predict_stress_score(features: &[f32; NUM_FEATURES]) -> f32 {
    let mut score = 0.0_f32;
    score += tree_0(features);
    score += tree_1(features);
    score += tree_2(features);
    score += tree_3(features);
    score += tree_4(features);
    score += tree_5(features);
    score += tree_6(features);
    score += tree_7(features);
    score += tree_8(features);
    score += tree_9(features);
    score += tree_10(features);
    score += tree_11(features);
    score += tree_12(features);
    score += tree_13(features);
    score += tree_14(features);
    score += tree_15(features);
    score += tree_16(features);
    score += tree_17(features);
    score += tree_18(features);
    score += tree_19(features);
    score += tree_20(features);
    score += tree_21(features);
    score += tree_22(features);
    score += tree_23(features);
    score += tree_24(features);
    score += tree_25(features);
    score += tree_26(features);
    score += tree_27(features);
    score += tree_28(features);
    score += tree_29(features);
    score += tree_30(features);
    score += tree_31(features);
    score += tree_32(features);
    score += tree_33(features);
    score += tree_34(features);
    score += tree_35(features);
    score += tree_36(features);
    score += tree_37(features);
    score += tree_38(features);
    score += tree_39(features);
    score += tree_40(features);
    score += tree_41(features);
    score += tree_42(features);
    score += tree_43(features);
    score += tree_44(features);
    1.0 / (1.0 + (-score).exp())
}

pub fn predict_crop_health(features: &[f32; NUM_FEATURES], threshold: f32) -> u8 {
    if predict_stress_score(features) >= threshold { 1 } else { 0 }
}
