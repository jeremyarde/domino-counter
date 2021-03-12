# To add a new cell, type '# %%'
# To add a new markdown cell, type '# %% [markdown]'
# %%
import sys
import cv2 as cv
import numpy as np


# %%
src = cv.imread(
    cv.samples.findFile("dominoes/IMG-20210306-WA0001.jpg"), cv.IMREAD_COLOR
)


# %%
gray = cv.cvtColor(src, cv.COLOR_BGR2GRAY)

gray = cv.medianBlur(gray, 5)

rows = gray.shape[0]
circles = cv.HoughCircles(
    gray,
    cv.HOUGH_GRADIENT,
    1,
    100,
    param1=100,
    param2=30,
    minRadius=1,
    maxRadius=10,
)


if circles is not None:
    circles = np.uint16(np.around(circles))
    for i in circles[0, :]:
        center = (i[0], i[1])
        # circle center
        cv.circle(src, center, 1, (0, 100, 100), 3)
        # circle outline
        radius = i[2]
        cv.circle(src, center, radius, (255, 0, 255), 3)


cv.imshow("detected circles", src)
cv.waitKey(0)


# %%
