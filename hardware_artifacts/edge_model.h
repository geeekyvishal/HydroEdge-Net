// HydroEdge-Net: Bare-Metal C Header for Microcontrollers
#ifndef HYDROEDGE_MODEL_H
#define HYDROEDGE_MODEL_H

#include <math.h>

#define NUM_FEATURES 15

static inline float tree_0(const float *features) {
    if (features[14] <= 1.985371f) {
        if (features[5] <= 25.860913f) {
            if (features[11] <= 3.433234f) {
                if (features[14] <= 1.629720f) {
                    if (features[10] <= 45.822155f) {
                        return 0.759972f;
                    } else {
                        return 0.910550f;
                    }
                } else {
                    return 0.923740f;
                }
            } else {
                if (features[11] <= 4.518059f) {
                    if (features[3] <= 28.448701f) {
                        return 0.747270f;
                    } else {
                        return 0.650599f;
                    }
                } else {
                    return 0.840559f;
                }
            }
        } else {
            if (features[13] <= 6.570718f) {
                if (features[6] <= 1.833727f) {
                    return 0.856856f;
                } else {
                    return 0.976001f;
                }
            } else {
                if (features[2] <= 30.838671f) {
                    if (features[5] <= 41.697684f) {
                        return 0.914952f;
                    } else {
                        return 0.775851f;
                    }
                } else {
                    return 0.768214f;
                }
            }
        }
    } else {
        if (features[5] <= 2.558450f) {
            if (features[4] <= 49.533796f) {
                return 0.644341f;
            } else {
                if (features[14] <= 3.849984f) {
                    return 0.789555f;
                } else {
                    return 0.670216f;
                }
            }
        } else {
            if (features[2] <= 11.740656f) {
                if (features[5] <= 33.162229f) {
                    return 0.948799f;
                } else {
                    return 0.800377f;
                }
            } else {
                if (features[14] <= 4.756457f) {
                    if (features[8] <= 0.101683f) {
                        return 0.747654f;
                    } else {
                        return 0.785200f;
                    }
                } else {
                    if (features[8] <= 0.079358f) {
                        return 0.930316f;
                    } else {
                        return 0.781322f;
                    }
                }
            }
        }
    }
}

static inline float tree_1(const float *features) {
    if (features[14] <= 1.985371f) {
        if (features[11] <= 2.046558f) {
            if (features[10] <= 37.338479f) {
                if (features[10] <= 28.597268f) {
                    return 0.011979f;
                } else {
                    return -0.095898f;
                }
            } else {
                if (features[14] <= 1.519938f) {
                    return 0.113649f;
                } else {
                    return 0.066928f;
                }
            }
        } else {
            if (features[0] <= 0.670170f) {
                if (features[0] <= 0.518319f) {
                    if (features[0] <= 0.489797f) {
                        return -0.039462f;
                    } else {
                        return -0.167464f;
                    }
                } else {
                    if (features[7] <= 14.484325f) {
                        return 0.093980f;
                    } else {
                        return -0.055593f;
                    }
                }
            } else {
                if (features[7] <= 13.133877f) {
                    if (features[14] <= 1.586685f) {
                        return -0.081761f;
                    } else {
                        return -0.183122f;
                    }
                } else {
                    return -0.015617f;
                }
            }
        }
    } else {
        if (features[7] <= 22.528436f) {
            if (features[12] <= 93.456601f) {
                if (features[2] <= 21.840348f) {
                    if (features[7] <= 16.171516f) {
                        return -0.011169f;
                    } else {
                        return -0.079042f;
                    }
                } else {
                    if (features[13] <= 5.815137f) {
                        return -0.146679f;
                    } else {
                        return -0.082531f;
                    }
                }
            } else {
                return -0.163931f;
            }
        } else {
            if (features[1] <= 0.402924f) {
                return 0.066610f;
            } else {
                if (features[10] <= 49.423397f) {
                    return 0.007666f;
                } else {
                    return -0.136928f;
                }
            }
        }
    }
}

static inline float tree_2(const float *features) {
    if (features[2] <= 21.840348f) {
        if (features[6] <= 0.798218f) {
            if (features[5] <= 16.239368f) {
                return -0.036798f;
            } else {
                return 0.055819f;
            }
        } else {
            if (features[12] <= 92.825489f) {
                if (features[6] <= 3.014147f) {
                    if (features[7] <= 22.528436f) {
                        return -0.042128f;
                    } else {
                        return 0.040196f;
                    }
                } else {
                    if (features[6] <= 4.266439f) {
                        return -0.111179f;
                    } else {
                        return 0.025339f;
                    }
                }
            } else {
                return -0.147963f;
            }
        }
    } else {
        if (features[5] <= 23.016209f) {
            if (features[5] <= 14.596674f) {
                if (features[5] <= 2.778065f) {
                    if (features[4] <= 56.322971f) {
                        return -0.166804f;
                    } else {
                        return -0.056161f;
                    }
                } else {
                    if (features[6] <= 0.705347f) {
                        return -0.127545f;
                    } else {
                        return -0.042510f;
                    }
                }
            } else {
                if (features[0] <= 0.397124f) {
                    if (features[10] <= 40.175076f) {
                        return -0.107532f;
                    } else {
                        return 0.018058f;
                    }
                } else {
                    return -0.137969f;
                }
            }
        } else {
            if (features[10] <= 53.043782f) {
                if (features[1] <= 0.171325f) {
                    return 0.013592f;
                } else {
                    if (features[4] <= 53.712699f) {
                        return -0.052930f;
                    } else {
                        return -0.092779f;
                    }
                }
            } else {
                if (features[1] <= 0.395604f) {
                    return 0.075142f;
                } else {
                    if (features[7] <= 6.858814f) {
                        return -0.088113f;
                    } else {
                        return 0.041133f;
                    }
                }
            }
        }
    }
}

static inline float tree_3(const float *features) {
    if (features[14] <= 1.985371f) {
        if (features[5] <= 25.860913f) {
            if (features[11] <= 3.433234f) {
                if (features[14] <= 1.629720f) {
                    if (features[10] <= 45.822155f) {
                        return -0.078101f;
                    } else {
                        return 0.047814f;
                    }
                } else {
                    if (features[14] <= 1.870767f) {
                        return 0.114505f;
                    } else {
                        return -0.012157f;
                    }
                }
            } else {
                if (features[11] <= 4.518059f) {
                    return -0.127058f;
                } else {
                    return -0.010302f;
                }
            }
        } else {
            if (features[13] <= 6.570718f) {
                if (features[8] <= 0.277851f) {
                    return 0.018880f;
                } else {
                    return 0.112576f;
                }
            } else {
                if (features[7] <= 4.219358f) {
                    return -0.071354f;
                } else {
                    if (features[13] <= 7.374752f) {
                        return 0.073615f;
                    } else {
                        return -0.049073f;
                    }
                }
            }
        }
    } else {
        if (features[5] <= 2.558450f) {
            if (features[10] <= 46.919190f) {
                return -0.088486f;
            } else {
                return -0.163528f;
            }
        } else {
            if (features[14] <= 4.756457f) {
                if (features[8] <= 0.101683f) {
                    if (features[13] <= 8.073638f) {
                        return -0.076853f;
                    } else {
                        return -0.142508f;
                    }
                } else {
                    if (features[5] <= 12.294200f) {
                        return -0.001727f;
                    } else {
                        return -0.062469f;
                    }
                }
            } else {
                if (features[8] <= 0.079358f) {
                    return 0.072574f;
                } else {
                    if (features[14] <= 4.876238f) {
                        return 0.033731f;
                    } else {
                        return -0.084936f;
                    }
                }
            }
        }
    }
}

static inline float tree_4(const float *features) {
    if (features[14] <= 1.840385f) {
        if (features[11] <= 2.046558f) {
            if (features[10] <= 37.338479f) {
                return 0.000977f;
            } else {
                return 0.110561f;
            }
        } else {
            if (features[12] <= 55.475978f) {
                if (features[10] <= 34.878473f) {
                    return -0.077005f;
                } else {
                    return 0.063302f;
                }
            } else {
                if (features[1] <= 0.307521f) {
                    return -0.009441f;
                } else {
                    return -0.075179f;
                }
            }
        }
    } else {
        if (features[4] <= 76.760762f) {
            if (features[14] <= 3.120642f) {
                if (features[2] <= 33.416672f) {
                    if (features[12] <= 92.825489f) {
                        return -0.057010f;
                    } else {
                        return -0.178160f;
                    }
                } else {
                    if (features[13] <= 6.324839f) {
                        return -0.168806f;
                    } else {
                        return -0.083577f;
                    }
                }
            } else {
                if (features[2] <= 12.066318f) {
                    return 0.033050f;
                } else {
                    if (features[6] <= 4.533028f) {
                        return -0.051079f;
                    } else {
                        return -0.117066f;
                    }
                }
            }
        } else {
            if (features[11] <= 3.628531f) {
                if (features[2] <= 30.683121f) {
                    if (features[0] <= 0.527751f) {
                        return 0.121599f;
                    } else {
                        return 0.004742f;
                    }
                } else {
                    if (features[0] <= 0.483926f) {
                        return -0.116508f;
                    } else {
                        return 0.067580f;
                    }
                }
            } else {
                if (features[12] <= 74.905154f) {
                    if (features[11] <= 4.338376f) {
                        return -0.137062f;
                    } else {
                        return -0.032985f;
                    }
                } else {
                    return 0.005387f;
                }
            }
        }
    }
}

static inline float tree_5(const float *features) {
    if (features[14] <= 2.220612f) {
        if (features[5] <= 25.860913f) {
            if (features[1] <= 0.676512f) {
                if (features[1] <= 0.636421f) {
                    if (features[10] <= 25.931685f) {
                        return -0.101077f;
                    } else {
                        return -0.029937f;
                    }
                } else {
                    return -0.149288f;
                }
            } else {
                return 0.026362f;
            }
        } else {
            if (features[1] <= 0.279130f) {
                if (features[14] <= 1.459039f) {
                    return 0.005552f;
                } else {
                    return 0.117427f;
                }
            } else {
                if (features[10] <= 29.175866f) {
                    if (features[7] <= 8.946685f) {
                        return -0.096749f;
                    } else {
                        return 0.012367f;
                    }
                } else {
                    if (features[13] <= 6.570718f) {
                        return 0.075493f;
                    } else {
                        return -0.017623f;
                    }
                }
            }
        }
    } else {
        if (features[14] <= 3.120642f) {
            if (features[4] <= 78.171242f) {
                if (features[8] <= 0.125200f) {
                    if (features[0] <= 0.324759f) {
                        return -0.035992f;
                    } else {
                        return -0.129517f;
                    }
                } else {
                    if (features[14] <= 2.998338f) {
                        return -0.056897f;
                    } else {
                        return -0.114505f;
                    }
                }
            } else {
                if (features[3] <= 31.509838f) {
                    return -0.060066f;
                } else {
                    return 0.122778f;
                }
            }
        } else {
            if (features[5] <= 2.558450f) {
                return -0.111432f;
            } else {
                if (features[7] <= 23.010121f) {
                    if (features[8] <= 1.291698f) {
                        return -0.039391f;
                    } else {
                        return -0.091873f;
                    }
                } else {
                    return 0.055442f;
                }
            }
        }
    }
}

static inline float tree_6(const float *features) {
    if (features[14] <= 1.985371f) {
        if (features[5] <= 25.860913f) {
            if (features[11] <= 3.433234f) {
                if (features[14] <= 1.629720f) {
                    if (features[1] <= 0.495206f) {
                        return 0.016564f;
                    } else {
                        return -0.078819f;
                    }
                } else {
                    if (features[14] <= 1.870767f) {
                        return 0.111724f;
                    } else {
                        return 0.000672f;
                    }
                }
            } else {
                if (features[11] <= 4.518059f) {
                    if (features[5] <= 18.195749f) {
                        return -0.074105f;
                    } else {
                        return -0.149737f;
                    }
                } else {
                    return -0.001406f;
                }
            }
        } else {
            if (features[13] <= 6.570718f) {
                if (features[6] <= 1.833727f) {
                    return -0.004980f;
                } else {
                    return 0.108060f;
                }
            } else {
                if (features[2] <= 30.838671f) {
                    if (features[6] <= 2.412020f) {
                        return 0.061673f;
                    } else {
                        return -0.053015f;
                    }
                } else {
                    return -0.056735f;
                }
            }
        }
    } else {
        if (features[5] <= 2.558450f) {
            if (features[4] <= 49.533796f) {
                return -0.127653f;
            } else {
                return -0.064922f;
            }
        } else {
            if (features[14] <= 4.756457f) {
                if (features[2] <= 44.284373f) {
                    if (features[1] <= 0.713271f) {
                        return -0.041103f;
                    } else {
                        return -0.077577f;
                    }
                } else {
                    return -0.125912f;
                }
            } else {
                if (features[8] <= 0.079358f) {
                    return 0.073301f;
                } else {
                    if (features[14] <= 4.876238f) {
                        return 0.036321f;
                    } else {
                        return -0.067547f;
                    }
                }
            }
        }
    }
}

static inline float tree_7(const float *features) {
    if (features[14] <= 2.220612f) {
        if (features[5] <= 35.513317f) {
            if (features[11] <= 2.920213f) {
                if (features[10] <= 50.093917f) {
                    if (features[11] <= 2.460428f) {
                        return -0.033851f;
                    } else {
                        return 0.061366f;
                    }
                } else {
                    return 0.094543f;
                }
            } else {
                if (features[1] <= 0.676512f) {
                    if (features[3] <= 26.592722f) {
                        return -0.022723f;
                    } else {
                        return -0.086832f;
                    }
                } else {
                    return 0.045735f;
                }
            }
        } else {
            if (features[1] <= 0.341165f) {
                return 0.094087f;
            } else {
                if (features[0] <= 0.441860f) {
                    return -0.055137f;
                } else {
                    if (features[4] <= 68.377230f) {
                        return 0.089466f;
                    } else {
                        return -0.037122f;
                    }
                }
            }
        }
    } else {
        if (features[14] <= 2.825105f) {
            if (features[3] <= 39.430182f) {
                if (features[8] <= 0.006308f) {
                    return -0.115908f;
                } else {
                    if (features[14] <= 2.625664f) {
                        return -0.029557f;
                    } else {
                        return -0.099189f;
                    }
                }
            } else {
                return 0.020835f;
            }
        } else {
            if (features[5] <= 6.097790f) {
                if (features[4] <= 49.533796f) {
                    return -0.112095f;
                } else {
                    return -0.046323f;
                }
            } else {
                if (features[5] <= 9.667882f) {
                    if (features[9] <= 0.000000f) {
                        return -0.020460f;
                    } else {
                        return 0.088403f;
                    }
                } else {
                    if (features[5] <= 21.669326f) {
                        return -0.062211f;
                    } else {
                        return -0.026299f;
                    }
                }
            }
        }
    }
}

static inline float tree_8(const float *features) {
    if (features[2] <= 21.840348f) {
        if (features[13] <= 6.240687f) {
            if (features[0] <= 0.605080f) {
                if (features[10] <= 41.718705f) {
                    if (features[9] <= 0.000000f) {
                        return -0.014958f;
                    } else {
                        return 0.122495f;
                    }
                } else {
                    if (features[11] <= 3.758831f) {
                        return -0.024727f;
                    } else {
                        return -0.127484f;
                    }
                }
            } else {
                if (features[0] <= 0.775411f) {
                    return 0.126405f;
                } else {
                    return 0.022768f;
                }
            }
        } else {
            if (features[1] <= 0.752972f) {
                if (features[1] <= 0.413155f) {
                    if (features[2] <= 12.763372f) {
                        return 0.072393f;
                    } else {
                        return -0.025674f;
                    }
                } else {
                    if (features[2] <= 19.848333f) {
                        return -0.074689f;
                    } else {
                        return 0.012615f;
                    }
                }
            } else {
                return 0.067050f;
            }
        }
    } else {
        if (features[14] <= 4.910260f) {
            if (features[14] <= 2.220612f) {
                if (features[5] <= 41.944469f) {
                    if (features[1] <= 0.678385f) {
                        return -0.042087f;
                    } else {
                        return 0.029908f;
                    }
                } else {
                    if (features[0] <= 0.504697f) {
                        return -0.000391f;
                    } else {
                        return 0.119056f;
                    }
                }
            } else {
                if (features[1] <= 0.734269f) {
                    if (features[13] <= 5.838929f) {
                        return -0.091192f;
                    } else {
                        return -0.039050f;
                    }
                } else {
                    if (features[12] <= 76.816732f) {
                        return -0.115763f;
                    } else {
                        return -0.037736f;
                    }
                }
            }
        } else {
            return 0.045898f;
        }
    }
}

static inline float tree_9(const float *features) {
    if (features[5] <= 23.016209f) {
        if (features[2] <= 11.740656f) {
            return 0.059857f;
        } else {
            if (features[5] <= 11.030316f) {
                if (features[5] <= 6.097790f) {
                    if (features[0] <= 0.739531f) {
                        return -0.069099f;
                    } else {
                        return 0.010269f;
                    }
                } else {
                    if (features[7] <= 14.882920f) {
                        return -0.007861f;
                    } else {
                        return 0.127286f;
                    }
                }
            } else {
                if (features[12] <= 45.516423f) {
                    if (features[4] <= 58.750458f) {
                        return -0.054538f;
                    } else {
                        return 0.045968f;
                    }
                } else {
                    if (features[0] <= 0.397124f) {
                        return -0.039645f;
                    } else {
                        return -0.084233f;
                    }
                }
            }
        }
    } else {
        if (features[1] <= 0.263806f) {
            if (features[5] <= 43.278260f) {
                if (features[5] <= 39.431705f) {
                    if (features[5] <= 33.487388f) {
                        return -0.007845f;
                    } else {
                        return 0.063525f;
                    }
                } else {
                    return -0.075624f;
                }
            } else {
                if (features[5] <= 46.825767f) {
                    return 0.127042f;
                } else {
                    return -0.003442f;
                }
            }
        } else {
            if (features[14] <= 1.685886f) {
                if (features[3] <= 33.204626f) {
                    if (features[8] <= 0.357558f) {
                        return 0.009281f;
                    } else {
                        return 0.111846f;
                    }
                } else {
                    return -0.047933f;
                }
            } else {
                if (features[14] <= 3.348979f) {
                    if (features[4] <= 80.485824f) {
                        return -0.066614f;
                    } else {
                        return 0.024444f;
                    }
                } else {
                    return -0.020401f;
                }
            }
        }
    }
}

static inline float tree_10(const float *features) {
    if (features[14] <= 1.840385f) {
        if (features[11] <= 2.046558f) {
            if (features[5] <= 20.146559f) {
                return -0.012056f;
            } else {
                return 0.079898f;
            }
        } else {
            if (features[12] <= 55.475978f) {
                if (features[10] <= 34.878473f) {
                    return -0.051927f;
                } else {
                    return 0.062240f;
                }
            } else {
                if (features[10] <= 48.503653f) {
                    return -0.021543f;
                } else {
                    return -0.075464f;
                }
            }
        }
    } else {
        if (features[4] <= 58.427374f) {
            if (features[4] <= 52.938025f) {
                if (features[1] <= 0.441320f) {
                    if (features[10] <= 42.369606f) {
                        return 0.011817f;
                    } else {
                        return -0.037395f;
                    }
                } else {
                    if (features[12] <= 81.056449f) {
                        return -0.063280f;
                    } else {
                        return 0.013499f;
                    }
                }
            } else {
                if (features[5] <= 36.425166f) {
                    if (features[13] <= 7.454308f) {
                        return -0.043072f;
                    } else {
                        return -0.114129f;
                    }
                } else {
                    return -0.136804f;
                }
            }
        } else {
            if (features[7] <= 12.618089f) {
                if (features[11] <= 4.152200f) {
                    if (features[2] <= 43.477716f) {
                        return -0.013719f;
                    } else {
                        return -0.105304f;
                    }
                } else {
                    if (features[11] <= 4.338376f) {
                        return -0.133511f;
                    } else {
                        return -0.045243f;
                    }
                }
            } else {
                if (features[1] <= 0.724538f) {
                    if (features[14] <= 2.796279f) {
                        return -0.054546f;
                    } else {
                        return 0.021254f;
                    }
                } else {
                    return 0.123859f;
                }
            }
        }
    }
}

static inline float tree_11(const float *features) {
    if (features[12] <= 93.456601f) {
        if (features[2] <= 21.840348f) {
            if (features[0] <= 0.190111f) {
                if (features[13] <= 6.828592f) {
                    return 0.003873f;
                } else {
                    return -0.116634f;
                }
            } else {
                if (features[3] <= 40.503059f) {
                    if (features[6] <= 3.841741f) {
                        return -0.003921f;
                    } else {
                        return 0.083420f;
                    }
                } else {
                    return -0.068242f;
                }
            }
        } else {
            if (features[11] <= 1.056067f) {
                return 0.088320f;
            } else {
                if (features[12] <= 91.516911f) {
                    if (features[8] <= 0.536548f) {
                        return -0.029815f;
                    } else {
                        return -0.062679f;
                    }
                } else {
                    return 0.039678f;
                }
            }
        }
    } else {
        return -0.083828f;
    }
}

static inline float tree_12(const float *features) {
    if (features[10] <= 20.849390f) {
        return -0.079497f;
    } else {
        if (features[10] <= 21.276607f) {
            return 0.087474f;
        } else {
            if (features[1] <= 0.439071f) {
                if (features[1] <= 0.424917f) {
                    if (features[2] <= 12.763372f) {
                        return 0.047934f;
                    } else {
                        return -0.020950f;
                    }
                } else {
                    return 0.085027f;
                }
            } else {
                if (features[11] <= 1.960968f) {
                    if (features[6] <= 3.559705f) {
                        return 0.019184f;
                    } else {
                        return -0.093371f;
                    }
                } else {
                    if (features[11] <= 2.972920f) {
                        return -0.063967f;
                    } else {
                        return -0.030477f;
                    }
                }
            }
        }
    }
}

static inline float tree_13(const float *features) {
    if (features[14] <= 1.194441f) {
        if (features[4] <= 52.777888f) {
            return 0.072440f;
        } else {
            if (features[14] <= 1.077696f) {
                return -0.060586f;
            } else {
                return 0.045420f;
            }
        }
    } else {
        if (features[4] <= 78.171242f) {
            if (features[5] <= 2.558450f) {
                if (features[12] <= 58.168421f) {
                    return -0.022865f;
                } else {
                    if (features[2] <= 26.384858f) {
                        return -0.037655f;
                    } else {
                        return -0.111896f;
                    }
                }
            } else {
                if (features[2] <= 44.204866f) {
                    if (features[12] <= 93.456601f) {
                        return -0.021805f;
                    } else {
                        return -0.081127f;
                    }
                } else {
                    return -0.083695f;
                }
            }
        } else {
            if (features[11] <= 3.628531f) {
                if (features[3] <= 36.290643f) {
                    if (features[5] <= 30.824872f) {
                        return 0.048709f;
                    } else {
                        return -0.028401f;
                    }
                } else {
                    return 0.105599f;
                }
            } else {
                if (features[5] <= 31.209263f) {
                    if (features[14] <= 3.209193f) {
                        return -0.028810f;
                    } else {
                        return -0.097159f;
                    }
                } else {
                    return 0.034584f;
                }
            }
        }
    }
}

static inline float tree_14(const float *features) {
    if (features[10] <= 20.849390f) {
        return -0.070553f;
    } else {
        if (features[14] <= 1.427380f) {
            if (features[1] <= 0.685180f) {
                if (features[13] <= 7.362895f) {
                    if (features[1] <= 0.594922f) {
                        return 0.047489f;
                    } else {
                        return -0.059445f;
                    }
                } else {
                    if (features[6] <= 2.412020f) {
                        return 0.001840f;
                    } else {
                        return -0.093164f;
                    }
                }
            } else {
                return 0.082408f;
            }
        } else {
            if (features[10] <= 21.276607f) {
                return 0.135382f;
            } else {
                if (features[4] <= 58.427374f) {
                    if (features[4] <= 56.169011f) {
                        return -0.026391f;
                    } else {
                        return -0.081660f;
                    }
                } else {
                    if (features[13] <= 5.691085f) {
                        return -0.071289f;
                    } else {
                        return -0.007298f;
                    }
                }
            }
        }
    }
}

static inline float tree_15(const float *features) {
    if (features[5] <= 33.648645f) {
        if (features[10] <= 20.849390f) {
            return -0.093459f;
        } else {
            if (features[11] <= 3.867279f) {
                if (features[6] <= 0.525420f) {
                    return 0.069656f;
                } else {
                    if (features[8] <= 0.985013f) {
                        return -0.025679f;
                    } else {
                        return 0.012923f;
                    }
                }
            } else {
                if (features[7] <= 19.985700f) {
                    if (features[11] <= 4.502551f) {
                        return -0.062790f;
                    } else {
                        return -0.020462f;
                    }
                } else {
                    if (features[12] <= 65.549838f) {
                        return -0.000495f;
                    } else {
                        return 0.061817f;
                    }
                }
            }
        }
    } else {
        if (features[0] <= 0.238598f) {
            if (features[6] <= 1.871146f) {
                if (features[14] <= 3.332056f) {
                    return -0.065284f;
                } else {
                    return 0.059202f;
                }
            } else {
                if (features[5] <= 40.942677f) {
                    return -0.028304f;
                } else {
                    return -0.107155f;
                }
            }
        } else {
            if (features[0] <= 0.307847f) {
                if (features[2] <= 33.416672f) {
                    return 0.129692f;
                } else {
                    return 0.030898f;
                }
            } else {
                if (features[11] <= 2.934486f) {
                    if (features[11] <= 2.474923f) {
                        return -0.010490f;
                    } else {
                        return -0.073988f;
                    }
                } else {
                    if (features[4] <= 76.014651f) {
                        return 0.003920f;
                    } else {
                        return 0.074701f;
                    }
                }
            }
        }
    }
}

static inline float tree_16(const float *features) {
    if (features[12] <= 93.456601f) {
        if (features[5] <= 2.558450f) {
            if (features[14] <= 3.643830f) {
                if (features[12] <= 81.651323f) {
                    if (features[10] <= 46.057144f) {
                        return 0.044636f;
                    } else {
                        return -0.040123f;
                    }
                } else {
                    return -0.085156f;
                }
            } else {
                return -0.087119f;
            }
        } else {
            if (features[12] <= 76.654468f) {
                if (features[1] <= 0.554286f) {
                    if (features[2] <= 44.204866f) {
                        return -0.006411f;
                    } else {
                        return -0.096289f;
                    }
                } else {
                    if (features[1] <= 0.652397f) {
                        return -0.059799f;
                    } else {
                        return -0.019158f;
                    }
                }
            } else {
                if (features[1] <= 0.679851f) {
                    if (features[4] <= 64.030795f) {
                        return -0.023027f;
                    } else {
                        return 0.027146f;
                    }
                } else {
                    if (features[2] <= 31.856428f) {
                        return 0.099476f;
                    } else {
                        return 0.000133f;
                    }
                }
            }
        }
    } else {
        return -0.064431f;
    }
}

static inline float tree_17(const float *features) {
    if (features[0] <= 0.872582f) {
        if (features[0] <= 0.863192f) {
            if (features[12] <= 92.825489f) {
                if (features[2] <= 11.393150f) {
                    if (features[11] <= 3.210688f) {
                        return -0.011806f;
                    } else {
                        return 0.113203f;
                    }
                } else {
                    if (features[12] <= 74.517451f) {
                        return -0.021909f;
                    } else {
                        return -0.002280f;
                    }
                }
            } else {
                if (features[6] <= 2.115377f) {
                    return -0.087939f;
                } else {
                    return 0.036063f;
                }
            }
        } else {
            return -0.084304f;
        }
    } else {
        if (features[12] <= 76.184035f) {
            return 0.063858f;
        } else {
            return -0.020571f;
        }
    }
}

static inline float tree_18(const float *features) {
    if (features[5] <= 23.016209f) {
        if (features[2] <= 11.740656f) {
            return 0.057686f;
        } else {
            if (features[5] <= 11.030316f) {
                if (features[5] <= 6.097790f) {
                    if (features[5] <= 5.528014f) {
                        return -0.023002f;
                    } else {
                        return -0.106410f;
                    }
                } else {
                    if (features[7] <= 14.882920f) {
                        return 0.008151f;
                    } else {
                        return 0.127861f;
                    }
                }
            } else {
                if (features[13] <= 7.103067f) {
                    return -0.051741f;
                } else {
                    if (features[1] <= 0.460479f) {
                        return 0.017833f;
                    } else {
                        return -0.042282f;
                    }
                }
            }
        }
    } else {
        if (features[1] <= 0.263806f) {
            if (features[5] <= 43.278260f) {
                if (features[13] <= 7.623214f) {
                    if (features[11] <= 4.046989f) {
                        return 0.005685f;
                    } else {
                        return 0.126827f;
                    }
                } else {
                    return -0.046884f;
                }
            } else {
                if (features[5] <= 46.825767f) {
                    return 0.126776f;
                } else {
                    return 0.005170f;
                }
            }
        } else {
            if (features[0] <= 0.249255f) {
                if (features[1] <= 0.626024f) {
                    return -0.063333f;
                } else {
                    if (features[8] <= 0.728643f) {
                        return 0.045728f;
                    } else {
                        return -0.075860f;
                    }
                }
            } else {
                if (features[0] <= 0.306338f) {
                    if (features[0] <= 0.272598f) {
                        return 0.011088f;
                    } else {
                        return 0.136581f;
                    }
                } else {
                    if (features[11] <= 1.393125f) {
                        return -0.061825f;
                    } else {
                        return -0.006071f;
                    }
                }
            }
        }
    }
}

static inline float tree_19(const float *features) {
    if (features[14] <= 2.220612f) {
        if (features[11] <= 2.920213f) {
            if (features[11] <= 2.714720f) {
                if (features[10] <= 50.093917f) {
                    if (features[2] <= 36.475756f) {
                        return -0.033425f;
                    } else {
                        return 0.059239f;
                    }
                } else {
                    if (features[3] <= 27.710079f) {
                        return 0.018497f;
                    } else {
                        return 0.118115f;
                    }
                }
            } else {
                return 0.126498f;
            }
        } else {
            if (features[11] <= 3.086444f) {
                return -0.086311f;
            } else {
                if (features[2] <= 42.081790f) {
                    if (features[11] <= 3.433234f) {
                        return 0.075189f;
                    } else {
                        return -0.009526f;
                    }
                } else {
                    return -0.079057f;
                }
            }
        }
    } else {
        if (features[14] <= 2.863004f) {
            if (features[3] <= 35.950688f) {
                if (features[0] <= 0.208208f) {
                    return -0.095596f;
                } else {
                    if (features[14] <= 2.768709f) {
                        return -0.027363f;
                    } else {
                        return -0.099404f;
                    }
                }
            } else {
                if (features[9] <= 0.000000f) {
                    return 0.076724f;
                } else {
                    return -0.042280f;
                }
            }
        } else {
            if (features[11] <= 4.732218f) {
                if (features[11] <= 4.481266f) {
                    if (features[10] <= 23.205485f) {
                        return -0.055154f;
                    } else {
                        return -0.008199f;
                    }
                } else {
                    if (features[14] <= 4.214981f) {
                        return 0.099555f;
                    } else {
                        return -0.001912f;
                    }
                }
            } else {
                if (features[3] <= 32.950678f) {
                    return -0.089143f;
                } else {
                    return -0.001611f;
                }
            }
        }
    }
}

static inline float tree_20(const float *features) {
    if (features[5] <= 23.016209f) {
        if (features[2] <= 11.740656f) {
            return 0.055619f;
        } else {
            if (features[1] <= 0.139748f) {
                if (features[10] <= 39.122303f) {
                    return -0.109630f;
                } else {
                    return -0.011998f;
                }
            } else {
                if (features[5] <= 11.030316f) {
                    if (features[5] <= 6.097790f) {
                        return -0.025608f;
                    } else {
                        return 0.029833f;
                    }
                } else {
                    if (features[13] <= 7.567204f) {
                        return -0.040276f;
                    } else {
                        return 0.002350f;
                    }
                }
            }
        }
    } else {
        if (features[13] <= 7.416331f) {
            if (features[1] <= 0.294361f) {
                if (features[7] <= 11.704962f) {
                    if (features[13] <= 6.701189f) {
                        return -0.026905f;
                    } else {
                        return 0.072520f;
                    }
                } else {
                    return 0.081483f;
                }
            } else {
                if (features[8] <= 2.283553f) {
                    if (features[3] <= 32.200915f) {
                        return -0.015754f;
                    } else {
                        return 0.020837f;
                    }
                } else {
                    return -0.093962f;
                }
            }
        } else {
            if (features[12] <= 76.051737f) {
                if (features[12] <= 62.350008f) {
                    if (features[11] <= 2.682925f) {
                        return -0.062315f;
                    } else {
                        return 0.013688f;
                    }
                } else {
                    if (features[13] <= 7.814320f) {
                        return -0.114762f;
                    } else {
                        return -0.038539f;
                    }
                }
            } else {
                if (features[10] <= 46.305449f) {
                    if (features[12] <= 89.620908f) {
                        return 0.016505f;
                    } else {
                        return -0.078428f;
                    }
                } else {
                    return 0.109908f;
                }
            }
        }
    }
}

static inline float tree_21(const float *features) {
    if (features[14] <= 1.985371f) {
        if (features[13] <= 5.932073f) {
            if (features[11] <= 4.031317f) {
                return 0.093567f;
            } else {
                return -0.020426f;
            }
        } else {
            if (features[10] <= 25.931685f) {
                if (features[10] <= 24.174422f) {
                    return 0.002826f;
                } else {
                    return -0.091635f;
                }
            } else {
                if (features[11] <= 2.164962f) {
                    if (features[11] <= 1.257747f) {
                        return -0.013860f;
                    } else {
                        return 0.099940f;
                    }
                } else {
                    if (features[12] <= 62.562603f) {
                        return 0.024848f;
                    } else {
                        return -0.029578f;
                    }
                }
            }
        }
    } else {
        if (features[3] <= 41.253109f) {
            if (features[14] <= 2.056833f) {
                return -0.070246f;
            } else {
                if (features[14] <= 2.162343f) {
                    return 0.048963f;
                } else {
                    if (features[4] <= 58.427374f) {
                        return -0.021925f;
                    } else {
                        return -0.003042f;
                    }
                }
            }
        } else {
            return -0.079107f;
        }
    }
}

static inline float tree_22(const float *features) {
    if (features[6] <= 0.315798f) {
        return -0.078726f;
    } else {
        if (features[4] <= 83.915678f) {
            if (features[6] <= 0.445527f) {
                return 0.134084f;
            } else {
                if (features[4] <= 82.339434f) {
                    if (features[4] <= 80.318481f) {
                        return -0.012951f;
                    } else {
                        return 0.049677f;
                    }
                } else {
                    return -0.075728f;
                }
            }
        } else {
            if (features[10] <= 47.748916f) {
                if (features[13] <= 7.695972f) {
                    if (features[3] <= 30.914588f) {
                        return 0.077357f;
                    } else {
                        return -0.025490f;
                    }
                } else {
                    return -0.068172f;
                }
            } else {
                return 0.127104f;
            }
        }
    }
}

static inline float tree_23(const float *features) {
    if (features[0] <= 0.872582f) {
        if (features[0] <= 0.863192f) {
            if (features[1] <= 0.261085f) {
                if (features[13] <= 5.815137f) {
                    return -0.061483f;
                } else {
                    if (features[4] <= 60.996832f) {
                        return -0.008001f;
                    } else {
                        return 0.052562f;
                    }
                }
            } else {
                if (features[13] <= 7.416331f) {
                    if (features[12] <= 85.148382f) {
                        return -0.002602f;
                    } else {
                        return -0.039483f;
                    }
                } else {
                    if (features[13] <= 7.567204f) {
                        return -0.075710f;
                    } else {
                        return -0.011875f;
                    }
                }
            }
        } else {
            return -0.074959f;
        }
    } else {
        if (features[4] <= 60.285743f) {
            if (features[5] <= 32.079757f) {
                return -0.026228f;
            } else {
                return 0.039916f;
            }
        } else {
            return 0.091088f;
        }
    }
}

static inline float tree_24(const float *features) {
    if (features[5] <= 2.558450f) {
        if (features[14] <= 3.849984f) {
            if (features[8] <= 0.635539f) {
                if (features[1] <= 0.498621f) {
                    return -0.014372f;
                } else {
                    return 0.047837f;
                }
            } else {
                return -0.074199f;
            }
        } else {
            return -0.074204f;
        }
    } else {
        if (features[5] <= 4.390043f) {
            if (features[11] <= 3.254592f) {
                return 0.017549f;
            } else {
                return 0.094177f;
            }
        } else {
            if (features[5] <= 6.097790f) {
                if (features[1] <= 0.319911f) {
                    return -0.091914f;
                } else {
                    return -0.020428f;
                }
            } else {
                if (features[5] <= 11.030316f) {
                    if (features[7] <= 14.882920f) {
                        return 0.009740f;
                    } else {
                        return 0.121962f;
                    }
                } else {
                    if (features[5] <= 22.196541f) {
                        return -0.025837f;
                    } else {
                        return -0.003131f;
                    }
                }
            }
        }
    }
}

static inline float tree_25(const float *features) {
    if (features[12] <= 93.456601f) {
        if (features[12] <= 76.654468f) {
            if (features[1] <= 0.439071f) {
                if (features[1] <= 0.418478f) {
                    if (features[0] <= 0.489797f) {
                        return 0.012604f;
                    } else {
                        return -0.018060f;
                    }
                } else {
                    return 0.093287f;
                }
            } else {
                if (features[6] <= 2.680549f) {
                    if (features[3] <= 38.854802f) {
                        return -0.017220f;
                    } else {
                        return 0.091081f;
                    }
                } else {
                    if (features[3] <= 33.693266f) {
                        return -0.086248f;
                    } else {
                        return -0.019557f;
                    }
                }
            }
        } else {
            if (features[12] <= 77.536344f) {
                return 0.092040f;
            } else {
                if (features[5] <= 6.528847f) {
                    if (features[14] <= 3.905412f) {
                        return -0.011992f;
                    } else {
                        return -0.089455f;
                    }
                } else {
                    if (features[6] <= 3.970222f) {
                        return 0.013113f;
                    } else {
                        return -0.056623f;
                    }
                }
            }
        }
    } else {
        return -0.048438f;
    }
}

static inline float tree_26(const float *features) {
    if (features[6] <= 0.315798f) {
        return -0.069741f;
    } else {
        if (features[4] <= 83.915678f) {
            if (features[6] <= 0.445527f) {
                return 0.128805f;
            } else {
                if (features[4] <= 82.339434f) {
                    if (features[4] <= 80.318481f) {
                        return -0.009618f;
                    } else {
                        return 0.047534f;
                    }
                } else {
                    return -0.067553f;
                }
            }
        } else {
            if (features[4] <= 88.658720f) {
                if (features[14] <= 3.860273f) {
                    if (features[0] <= 0.369400f) {
                        return 0.007542f;
                    } else {
                        return 0.110954f;
                    }
                } else {
                    return -0.003780f;
                }
            } else {
                return -0.081512f;
            }
        }
    }
}

static inline float tree_27(const float *features) {
    if (features[10] <= 20.849390f) {
        return -0.046860f;
    } else {
        if (features[14] <= 1.427380f) {
            if (features[12] <= 47.129381f) {
                return 0.079685f;
            } else {
                if (features[5] <= 21.181974f) {
                    if (features[5] <= 14.053582f) {
                        return 0.050842f;
                    } else {
                        return -0.095248f;
                    }
                } else {
                    if (features[3] <= 33.204626f) {
                        return 0.062945f;
                    } else {
                        return -0.025853f;
                    }
                }
            }
        } else {
            if (features[10] <= 21.276607f) {
                return 0.133943f;
            } else {
                if (features[14] <= 4.756457f) {
                    if (features[7] <= 1.576472f) {
                        return -0.022728f;
                    } else {
                        return -0.004854f;
                    }
                } else {
                    if (features[2] <= 36.059599f) {
                        return -0.003848f;
                    } else {
                        return 0.100513f;
                    }
                }
            }
        }
    }
}

static inline float tree_28(const float *features) {
    if (features[0] <= 0.872582f) {
        if (features[0] <= 0.863192f) {
            if (features[1] <= 0.261085f) {
                if (features[13] <= 5.709677f) {
                    return -0.068069f;
                } else {
                    if (features[4] <= 60.996832f) {
                        return -0.004541f;
                    } else {
                        return 0.051915f;
                    }
                }
            } else {
                if (features[1] <= 0.304288f) {
                    if (features[10] <= 51.561577f) {
                        return -0.056287f;
                    } else {
                        return 0.035075f;
                    }
                } else {
                    if (features[14] <= 4.764161f) {
                        return -0.009742f;
                    } else {
                        return 0.032592f;
                    }
                }
            }
        } else {
            return -0.066970f;
        }
    } else {
        if (features[13] <= 6.465180f) {
            return -0.015644f;
        } else {
            return 0.058464f;
        }
    }
}

static inline float tree_29(const float *features) {
    if (features[12] <= 93.456601f) {
        if (features[13] <= 8.120718f) {
            if (features[1] <= 0.439071f) {
                if (features[13] <= 5.673264f) {
                    return -0.058056f;
                } else {
                    if (features[0] <= 0.489797f) {
                        return 0.023292f;
                    } else {
                        return -0.004868f;
                    }
                }
            } else {
                if (features[12] <= 75.864297f) {
                    if (features[12] <= 45.244936f) {
                        return 0.006327f;
                    } else {
                        return -0.029763f;
                    }
                } else {
                    if (features[9] <= 0.000000f) {
                        return 0.040700f;
                    } else {
                        return -0.008320f;
                    }
                }
            }
        } else {
            if (features[9] <= 0.000000f) {
                return -0.062498f;
            } else {
                return 0.004704f;
            }
        }
    } else {
        return -0.042786f;
    }
}

static inline float tree_30(const float *features) {
    if (features[5] <= 2.558450f) {
        if (features[10] <= 46.919190f) {
            if (features[3] <= 31.586538f) {
                return 0.035031f;
            } else {
                return -0.054812f;
            }
        } else {
            return -0.069698f;
        }
    } else {
        if (features[5] <= 4.390043f) {
            if (features[14] <= 3.702107f) {
                return 0.075468f;
            } else {
                return 0.017221f;
            }
        } else {
            if (features[5] <= 6.097790f) {
                if (features[10] <= 40.175076f) {
                    return -0.071385f;
                } else {
                    return -0.003675f;
                }
            } else {
                if (features[5] <= 11.030316f) {
                    if (features[7] <= 14.882920f) {
                        return 0.011880f;
                    } else {
                        return 0.118317f;
                    }
                } else {
                    if (features[5] <= 22.196541f) {
                        return -0.020653f;
                    } else {
                        return -0.000421f;
                    }
                }
            }
        }
    }
}

static inline float tree_31(const float *features) {
    if (features[10] <= 20.849390f) {
        return -0.041972f;
    } else {
        if (features[10] <= 21.276607f) {
            return 0.082503f;
        } else {
            if (features[14] <= 1.194441f) {
                if (features[10] <= 32.610383f) {
                    return 0.091667f;
                } else {
                    if (features[14] <= 1.077696f) {
                        return -0.025694f;
                    } else {
                        return 0.054169f;
                    }
                }
            } else {
                if (features[13] <= 8.046192f) {
                    if (features[13] <= 7.993712f) {
                        return -0.005425f;
                    } else {
                        return 0.052946f;
                    }
                } else {
                    if (features[2] <= 40.335109f) {
                        return -0.015376f;
                    } else {
                        return -0.076887f;
                    }
                }
            }
        }
    }
}

static inline float tree_32(const float *features) {
    if (features[6] <= 0.315798f) {
        return -0.062136f;
    } else {
        if (features[4] <= 83.915678f) {
            if (features[6] <= 0.445527f) {
                return 0.124936f;
            } else {
                if (features[4] <= 82.339434f) {
                    if (features[4] <= 80.318481f) {
                        return -0.006420f;
                    } else {
                        return 0.045324f;
                    }
                } else {
                    return -0.059186f;
                }
            }
        } else {
            if (features[4] <= 88.658720f) {
                if (features[14] <= 3.860273f) {
                    if (features[0] <= 0.349415f) {
                        return 0.004570f;
                    } else {
                        return 0.107218f;
                    }
                } else {
                    return -0.002358f;
                }
            } else {
                return -0.072924f;
            }
        }
    }
}

static inline float tree_33(const float *features) {
    if (features[12] <= 93.456601f) {
        if (features[5] <= 2.558450f) {
            if (features[12] <= 82.631682f) {
                if (features[10] <= 46.057144f) {
                    if (features[5] <= 1.349479f) {
                        return 0.074262f;
                    } else {
                        return -0.007666f;
                    }
                } else {
                    return -0.053032f;
                }
            } else {
                return -0.084934f;
            }
        } else {
            if (features[5] <= 4.390043f) {
                return 0.052316f;
            } else {
                if (features[5] <= 6.097790f) {
                    if (features[6] <= 2.115377f) {
                        return -0.011240f;
                    } else {
                        return -0.084900f;
                    }
                } else {
                    if (features[5] <= 11.030316f) {
                        return 0.028352f;
                    } else {
                        return -0.004297f;
                    }
                }
            }
        }
    } else {
        return -0.038787f;
    }
}

static inline float tree_34(const float *features) {
    if (features[2] <= 43.180975f) {
        if (features[2] <= 42.485272f) {
            if (features[4] <= 21.043213f) {
                return -0.050941f;
            } else {
                if (features[4] <= 25.841937f) {
                    if (features[14] <= 2.698554f) {
                        return 0.069510f;
                    } else {
                        return 0.005387f;
                    }
                } else {
                    if (features[4] <= 29.360987f) {
                        return -0.033337f;
                    } else {
                        return -0.003044f;
                    }
                }
            }
        } else {
            return 0.069495f;
        }
    } else {
        if (features[4] <= 50.977458f) {
            return 0.023649f;
        } else {
            if (features[5] <= 23.562051f) {
                return -0.114054f;
            } else {
                return -0.001755f;
            }
        }
    }
}

static inline float tree_35(const float *features) {
    if (features[0] <= 0.872582f) {
        if (features[0] <= 0.863192f) {
            if (features[10] <= 20.849390f) {
                return -0.039340f;
            } else {
                if (features[14] <= 1.439031f) {
                    if (features[14] <= 1.329393f) {
                        return 0.004788f;
                    } else {
                        return 0.060385f;
                    }
                } else {
                    if (features[0] <= 0.414237f) {
                        return 0.003704f;
                    } else {
                        return -0.010941f;
                    }
                }
            }
        } else {
            return -0.058607f;
        }
    } else {
        if (features[12] <= 76.184035f) {
            return 0.061223f;
        } else {
            return -0.013394f;
        }
    }
}

static inline float tree_36(const float *features) {
    if (features[6] <= 0.315798f) {
        return -0.057848f;
    } else {
        if (features[4] <= 83.915678f) {
            if (features[6] <= 0.445527f) {
                return 0.121341f;
            } else {
                if (features[6] <= 1.010745f) {
                    if (features[2] <= 41.546679f) {
                        return -0.011121f;
                    } else {
                        return -0.091773f;
                    }
                } else {
                    if (features[4] <= 77.405639f) {
                        return -0.003608f;
                    } else {
                        return 0.056665f;
                    }
                }
            }
        } else {
            if (features[10] <= 47.748916f) {
                if (features[12] <= 82.631682f) {
                    if (features[11] <= 3.867279f) {
                        return 0.059735f;
                    } else {
                        return -0.050106f;
                    }
                } else {
                    return -0.069497f;
                }
            } else {
                return 0.117110f;
            }
        }
    }
}

static inline float tree_37(const float *features) {
    if (features[1] <= 0.117558f) {
        if (features[3] <= 33.951421f) {
            return -0.069219f;
        } else {
            return 0.015288f;
        }
    } else {
        if (features[1] <= 0.121344f) {
            return 0.140251f;
        } else {
            if (features[0] <= 0.872582f) {
                if (features[1] <= 0.441320f) {
                    if (features[13] <= 5.660034f) {
                        return -0.065332f;
                    } else {
                        return 0.007789f;
                    }
                } else {
                    if (features[11] <= 1.960968f) {
                        return 0.018359f;
                    } else {
                        return -0.017185f;
                    }
                }
            } else {
                if (features[13] <= 6.465180f) {
                    return -0.015651f;
                } else {
                    return 0.067071f;
                }
            }
        }
    }
}

static inline float tree_38(const float *features) {
    if (features[2] <= 24.688549f) {
        if (features[6] <= 0.789858f) {
            if (features[5] <= 16.239368f) {
                return -0.006871f;
            } else {
                if (features[13] <= 7.148197f) {
                    return 0.105268f;
                } else {
                    return 0.029221f;
                }
            }
        } else {
            if (features[6] <= 0.860317f) {
                return -0.067612f;
            } else {
                if (features[0] <= 0.199584f) {
                    if (features[13] <= 7.103067f) {
                        return 0.006434f;
                    } else {
                        return -0.064765f;
                    }
                } else {
                    if (features[12] <= 54.344237f) {
                        return 0.032332f;
                    } else {
                        return -0.001577f;
                    }
                }
            }
        }
    } else {
        if (features[8] <= 0.516559f) {
            if (features[3] <= 33.879918f) {
                if (features[14] <= 4.671106f) {
                    if (features[2] <= 30.262466f) {
                        return 0.015079f;
                    } else {
                        return -0.023429f;
                    }
                } else {
                    if (features[6] <= 1.444567f) {
                        return -0.001225f;
                    } else {
                        return 0.102315f;
                    }
                }
            } else {
                if (features[6] <= 4.533028f) {
                    if (features[6] <= 3.431040f) {
                        return 0.018554f;
                    } else {
                        return 0.086389f;
                    }
                } else {
                    return -0.053576f;
                }
            }
        } else {
            if (features[3] <= 38.786889f) {
                if (features[0] <= 0.507113f) {
                    return -0.020191f;
                } else {
                    return -0.094497f;
                }
            } else {
                return -0.002735f;
            }
        }
    }
}

static inline float tree_39(const float *features) {
    if (features[11] <= 1.377705f) {
        if (features[3] <= 26.367544f) {
            if (features[3] <= 22.956895f) {
                if (features[4] <= 55.361541f) {
                    return -0.057419f;
                } else {
                    return 0.047757f;
                }
            } else {
                return -0.097876f;
            }
        } else {
            if (features[6] <= 2.861364f) {
                if (features[10] <= 50.923152f) {
                    if (features[11] <= 1.154902f) {
                        return 0.140583f;
                    } else {
                        return 0.021691f;
                    }
                } else {
                    return -0.040870f;
                }
            } else {
                return -0.053007f;
            }
        }
    } else {
        if (features[11] <= 1.851344f) {
            if (features[13] <= 5.986383f) {
                return -0.028322f;
            } else {
                if (features[13] <= 7.119653f) {
                    if (features[7] <= 14.569240f) {
                        return 0.128780f;
                    } else {
                        return 0.017696f;
                    }
                } else {
                    if (features[14] <= 4.329161f) {
                        return 0.047791f;
                    } else {
                        return -0.060461f;
                    }
                }
            }
        } else {
            if (features[1] <= 0.439071f) {
                if (features[1] <= 0.418478f) {
                    if (features[3] <= 37.812277f) {
                        return 0.009463f;
                    } else {
                        return -0.029700f;
                    }
                } else {
                    return 0.074976f;
                }
            } else {
                if (features[5] <= 33.648645f) {
                    if (features[12] <= 75.591714f) {
                        return -0.029767f;
                    } else {
                        return 0.000151f;
                    }
                } else {
                    if (features[11] <= 3.056258f) {
                        return -0.036545f;
                    } else {
                        return 0.034589f;
                    }
                }
            }
        }
    }
}

static inline float tree_40(const float *features) {
    if (features[10] <= 20.849390f) {
        return -0.034753f;
    } else {
        if (features[10] <= 21.276607f) {
            return 0.081477f;
        } else {
            if (features[14] <= 1.194441f) {
                if (features[4] <= 71.660200f) {
                    if (features[13] <= 7.362895f) {
                        return 0.116754f;
                    } else {
                        return 0.006247f;
                    }
                } else {
                    return -0.029206f;
                }
            } else {
                if (features[8] <= 2.569556f) {
                    if (features[8] <= 1.935877f) {
                        return -0.002377f;
                    } else {
                        return -0.053089f;
                    }
                } else {
                    return 0.051155f;
                }
            }
        }
    }
}

static inline float tree_41(const float *features) {
    if (features[12] <= 93.456601f) {
        if (features[12] <= 76.654468f) {
            if (features[12] <= 68.394533f) {
                if (features[0] <= 0.249255f) {
                    if (features[1] <= 0.554286f) {
                        return -0.004154f;
                    } else {
                        return -0.055474f;
                    }
                } else {
                    if (features[7] <= 10.327402f) {
                        return -0.001985f;
                    } else {
                        return 0.024944f;
                    }
                }
            } else {
                if (features[10] <= 40.058869f) {
                    if (features[0] <= 0.760447f) {
                        return 0.024154f;
                    } else {
                        return -0.086988f;
                    }
                } else {
                    if (features[12] <= 72.051917f) {
                        return -0.093467f;
                    } else {
                        return -0.020270f;
                    }
                }
            }
        } else {
            if (features[1] <= 0.679851f) {
                if (features[1] <= 0.143404f) {
                    return -0.051310f;
                } else {
                    if (features[5] <= 6.528847f) {
                        return -0.041907f;
                    } else {
                        return 0.015421f;
                    }
                }
            } else {
                if (features[8] <= 0.272396f) {
                    return 0.002287f;
                } else {
                    if (features[7] <= 16.434629f) {
                        return 0.135896f;
                    } else {
                        return 0.064261f;
                    }
                }
            }
        }
    } else {
        return -0.033998f;
    }
}

static inline float tree_42(const float *features) {
    if (features[0] <= 0.411399f) {
        if (features[5] <= 2.019150f) {
            return -0.049675f;
        } else {
            if (features[3] <= 20.067023f) {
                if (features[5] <= 19.170990f) {
                    return -0.075676f;
                } else {
                    return 0.015494f;
                }
            } else {
                if (features[0] <= 0.249255f) {
                    if (features[5] <= 18.944665f) {
                        return 0.047831f;
                    } else {
                        return -0.021331f;
                    }
                } else {
                    if (features[2] <= 29.554313f) {
                        return 0.060906f;
                    } else {
                        return 0.002132f;
                    }
                }
            }
        }
    } else {
        if (features[11] <= 1.393125f) {
            if (features[0] <= 0.741570f) {
                if (features[11] <= 1.191769f) {
                    return -0.019617f;
                } else {
                    return -0.084984f;
                }
            } else {
                return 0.016383f;
            }
        } else {
            if (features[11] <= 1.960968f) {
                if (features[1] <= 0.433176f) {
                    if (features[0] <= 0.695754f) {
                        return -0.058018f;
                    } else {
                        return 0.073992f;
                    }
                } else {
                    if (features[5] <= 21.796406f) {
                        return 0.054102f;
                    } else {
                        return 0.118379f;
                    }
                }
            } else {
                if (features[3] <= 18.947701f) {
                    return 0.063563f;
                } else {
                    if (features[7] <= 13.387204f) {
                        return -0.018146f;
                    } else {
                        return 0.009498f;
                    }
                }
            }
        }
    }
}

static inline float tree_43(const float *features) {
    if (features[8] <= 1.935877f) {
        if (features[7] <= 22.528436f) {
            if (features[2] <= 12.544509f) {
                return -0.117616f;
            } else {
                if (features[2] <= 12.763372f) {
                    return 0.128619f;
                } else {
                    if (features[2] <= 12.975397f) {
                        return -0.082545f;
                    } else {
                        return -0.001639f;
                    }
                }
            }
        } else {
            if (features[3] <= 26.367544f) {
                if (features[13] <= 7.406767f) {
                    return 0.040136f;
                } else {
                    return -0.066998f;
                }
            } else {
                if (features[14] <= 4.072897f) {
                    return 0.114842f;
                } else {
                    return 0.068154f;
                }
            }
        }
    } else {
        if (features[6] <= 4.211475f) {
            return -0.062683f;
        } else {
            return 0.030114f;
        }
    }
}

static inline float tree_44(const float *features) {
    if (features[1] <= 0.776642f) {
        if (features[1] <= 0.652397f) {
            if (features[1] <= 0.439071f) {
                if (features[13] <= 5.709677f) {
                    if (features[6] <= 1.925396f) {
                        return -0.072698f;
                    } else {
                        return 0.028903f;
                    }
                } else {
                    if (features[3] <= 41.253109f) {
                        return 0.009169f;
                    } else {
                        return -0.075056f;
                    }
                }
            } else {
                if (features[4] <= 46.230307f) {
                    if (features[8] <= 1.141563f) {
                        return -0.048595f;
                    } else {
                        return 0.011914f;
                    }
                } else {
                    if (features[3] <= 38.786889f) {
                        return -0.010176f;
                    } else {
                        return 0.093863f;
                    }
                }
            }
        } else {
            if (features[1] <= 0.701514f) {
                if (features[7] <= 4.639482f) {
                    return 0.101973f;
                } else {
                    if (features[1] <= 0.679851f) {
                        return -0.014105f;
                    } else {
                        return 0.057709f;
                    }
                }
            } else {
                if (features[7] <= 8.059418f) {
                    if (features[10] <= 49.423397f) {
                        return -0.003683f;
                    } else {
                        return -0.071480f;
                    }
                } else {
                    if (features[8] <= 1.034241f) {
                        return 0.081275f;
                    } else {
                        return -0.048779f;
                    }
                }
            }
        }
    } else {
        if (features[1] <= 0.786792f) {
            return -0.061769f;
        } else {
            return 0.022629f;
        }
    }
}

static inline float predict_stress_score(const float *features) {
    float score = 0.0f;
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
    return 1.0f / (1.0f + expf(-score));
}

static inline int predict_crop_health(const float *features, float threshold) {
    return (predict_stress_score(features) >= threshold) ? 1 : 0;
}

#endif // HYDROEDGE_MODEL_H
