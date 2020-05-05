#!/usr/bin/env python

import unittest
from antbinding import Recording, Frame, Pose


class TestRecording(unittest.TestCase):

    def test_io(self):
        filename = '/tmp/recoding.bin'
        recording = Recording()
        recording.map.width = 35.
        recording.map.height = 53.
        recording.add_frame(Frame())
        recording.dump(filename)

        loaded = Recording.load(filename)
        self.assertEqual(len(loaded.frames), 1)

        self.assertEqual(loaded.map.width, 35.)
        self.assertEqual(loaded.map.height, 53.)

    def test_frame(self):
        frame = Frame()
        frame.add_ant(Pose(y=4., x=2.))

        self.assertEqual(frame.ants[0].x, 2.)
        self.assertEqual(frame.ants[0].y, 4.)


if __name__ == '__main__':
    unittest.main()
