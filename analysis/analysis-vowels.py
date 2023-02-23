import scipy
import numpy as np
import matplotlib.pyplot as plt

files = ['ʌ','a','aɪ.1','aɪ.2','æ','eɪ.1','eɪ.2','aʊ.1','aʊ.2',
         'ɛ','ɛə.1','ɛə.2','ɜ','ə','oʊ.1','oʊ.2','ɪ','ɪə.1','ɪə.2',
         'i','ɔ','ɔɪ.1','ɔɪ.2','ɒ','u','ʊ','m','v','ð','z','ʒ','ɹ','ŋ','n','w']

plot = False

def process_wav(file):
    samplerate, data = scipy.io.wavfile.read('./recordings/vowels2/' + file + '.wav')
    
    data_normalised = data / 32768.0
    N = len(data)
    T = 1.0/samplerate
    y = scipy.fft.fft(data_normalised);
    x = scipy.fft.fftfreq(N, T)[:N//2]
    
    window_size = 25
    #running_average = np.convolve(, np.ones(window_size)/window_size, mode='valid')
    real = 2.0/N * np.abs(y[0:N//2])
    maximum_conv = (scipy.ndimage.filters.maximum_filter(real, footprint=np.ones(window_size)))
    mean_recording_freq = 146
    search_range = 20
    search_step = 0.01
    
    record_xd = None
    record_yd = None
    record_total = 0
    for i in np.arange(mean_recording_freq - search_range, mean_recording_freq + search_range, search_step):
        xd = np.arange(i,20000,i)
        yd = maximum_conv[np.floor((1/x[1])*xd).astype(int)]
        total = yd.sum()
        if total > record_total:
            record_xd = xd
            record_yd = yd
            record_total = total

    output = "["
    for i in record_yd[:50]:
        output += "{:.8f}, ".format(i)

    output = output[:-2] + "],"
    print(output)
    
    if plot:
        figManager = plt.get_current_fig_manager()
        figManager.window.showMaximized()
        plt.plot(x[:N//2], maximum_conv)
        plt.plot(record_xd,record_yd,'ko')
        plt.xlim(0, 4000)
        plt.ylim(0, 0.3)
        plt.show()

for file in files:
    process_wav(file)
