#! python
# -*- coding: utf-8 -*-

"""
Model Reduction with
Farthest Point Search
"""

from mpl_toolkits.mplot3d import Axes3D

import os
import matplotlib.pyplot as plt
import numpy as np
import matplotlib as mpl
from scipy.spatial.distance import euclidean
import sys


def distances_from(reference, points):
    """
    Computes the distances of all points in a list from a reference point
    """
    return [euclidean(reference,p) for p in points]


def index_of_longest(distances):
    """
    Gets the index of the longest distance
    """
    ds_max = max(distances)
    return distances.index(ds_max)


if __name__ == "__main__":
    # Load the points in the file in first argument
    xyz_file = np.loadtxt(sys.argv[1])
    # separate x, y, z coordinates
    x = xyz_file[:,0]
    y = xyz_file[:,1]
    z = xyz_file[:,2]
    # the number of points in the original model
    points_count = len(x)
    # constructs the points in the original model
    points = [[x[i],y[i],z[i]] for i in range(points_count)]
    # the number of points to select
    target_count=2048
    # initializes the list of selected points
    farthest_points = [0]*target_count

    # select a random first point
    p0 = points[np.random.randint(0,points_count)]
    farthest_points[0]=p0
    distances = distances_from(p0,points)
    # select other points
    for i in range(1,target_count):
        # select the parthest point from all other
        farthest_points[i] = points[index_of_longest(distances)]
        distances_from_selected = distances_from(farthest_points[i],points)
        distances = [
            min(distances[j],distances_from_selected[j])
            for j in range(len(distances))
        ]

    # display the selected points
    xf=[farthest_points[j][0] for j in range(len(farthest_points))]
    yf=[farthest_points[j][1] for j in range(len(farthest_points))]
    zf=[farthest_points[j][2] for j in range(len(farthest_points))]
    fig = plt.figure()
    ax = fig.add_subplot(111, projection='3d')
    plt.grid(False)
    ax.scatter(zf, xf, yf, c='r',s=4)
    ax.set_xlabel('X Label')
    ax.set_ylabel('Y Label')
    ax.set_zlabel('Z Label')
    plt.show()
