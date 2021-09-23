#!/usr/bin/env python

import unittest
from antbinding import Recording, Frame, Pose, Ant, AntHill, SugarHill


class TestRecording(unittest.TestCase):
    def test_io(self):
        filename = '/tmp/recoding.bin'
        recording = Recording()
        recording.map.width = 35.0
        recording.map.height = 53.0
        recording.add_frame(Frame())
        recording.dump(filename)

        loaded = Recording.load(filename)
        self.assertEqual(len(loaded.frames), 1)

        self.assertEqual(loaded.map.width, 35.0)
        self.assertEqual(loaded.map.height, 53.0)

    def test_frame(self):
        frame = Frame()

        frame.add_ant(Ant(y=4.0, x=2.0))

        self.assertAlmostEqual(frame.ants[0].pose.x, 2.0)
        self.assertAlmostEqual(frame.ants[0].pose.y, 4.0)

        frame.add_anthill(AntHill(y=4.0, x=2.0, team=0))
        self.assertAlmostEqual(frame.anthills[0].pose.x, 2.0)
        self.assertAlmostEqual(frame.anthills[0].pose.y, 4.0)
        self.assertEqual(frame.anthills[0].team, 0)

        frame.add_raspberry(Pose(y=1.0, x=4.0, rotation=1.3))
        self.assertAlmostEqual(frame.raspberries[0].x, 4.0)
        self.assertAlmostEqual(frame.raspberries[0].y, 1.0)
        self.assertAlmostEqual(frame.raspberries[0].rotation, 1.3)

        frame.add_sugar_hill(SugarHill(y=2.0, x=3.0, rotation=1.3, volume=8.0))
        self.assertAlmostEqual(frame.sugar_hills[0].pose.x, 3.0)
        self.assertAlmostEqual(frame.sugar_hills[0].pose.y, 2.0)
        self.assertAlmostEqual(frame.sugar_hills[0].pose.rotation, 1.3)
        self.assertAlmostEqual(frame.sugar_hills[0].volume, 8.0)


if __name__ == '__main__':
    unittest.main()
