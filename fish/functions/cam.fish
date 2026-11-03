function cam --description 'Reset webcam (kill camera assistants so USB re-probes)'
    sudo killall VDCAssistant AppleCameraAssistant 2>/dev/null
    echo "Camera assistants killed. Reopen your camera app."
end
