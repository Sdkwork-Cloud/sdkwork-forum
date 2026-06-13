@echo off
set SDKWORK_FORUM_DATABASE_URL=postgresql://forum:forum123@localhost:5432/forum
cd /d E:\sdkwork-space\sdkwork-forum
target\debug\forum-server.exe
